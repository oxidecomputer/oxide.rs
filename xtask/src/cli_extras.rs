use indexmap::IndexSet;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use std::sync::OnceLock;
use syn::{
    Fields, GenericArgument, ImplItem, ImplItemFn, Index, Item, ItemEnum, ItemImpl, ItemStruct,
    PathArguments, ReturnType, Signature, Type, TypePath, TypeTuple, Variant,
};

static UNPRESENTABLE_TYPES: OnceLock<HashSet<&'static str>> = OnceLock::new();

fn is_unpresentable(type_name: &str) -> bool {
    UNPRESENTABLE_TYPES
        .get_or_init(|| {
            HashSet::from([
                "ByteStream",                       // Unstructured bytes
                "reqwest::Upgraded",                // HTTP connection
                "serde_json::Value",                // No defined field names
                "types::InstanceSerialConsoleData", // Unstructured bytes
            ])
        })
        .contains(type_name)
}

/// Generate a `ResponseFields` implementation for all types in the `oxide::types` module.
pub fn gen_response_fields(sdk_tokens: TokenStream, cli_tokens: TokenStream) -> TokenStream {
    let sdk_file: syn::File = syn::parse2(sdk_tokens).unwrap();
    let cli_file: syn::File = syn::parse2(cli_tokens).unwrap();

    let cli_cmd_tokens = gen_cli_command(&sdk_file, &cli_file);
    let response_field_tokens = gen_response_fields_impls(&sdk_file);

    quote! {

        #response_field_tokens

        #cli_cmd_tokens
    }
}

/// Generate a `field_names` method for `CliCommand` use to determine which subcommands
/// are eligible for the `--format` flag, and adding the list of available fields `--help`
/// output.
fn gen_cli_command(sdk_file: &syn::File, cli_file: &syn::File) -> TokenStream {
    let builder = sdk_file
        .items
        .iter()
        .filter_map(|i| match i {
            Item::Mod(module) => Some(module),
            _ => None,
        })
        .find(|m| m.ident == "builder")
        .unwrap();

    let Some((_, builder_content)) = &builder.content else {
        unreachable!();
    };

    let cli_variants = cli_file
        .items
        .iter()
        .filter_map(|i| match i {
            Item::Enum(ItemEnum {
                ident, variants, ..
            }) if ident == "CliCommand" => Some(variants),
            _ => None,
        })
        .next()
        .unwrap();

    let mut match_arms = Vec::new();
    for Variant {
        ident: variant_ident,
        ..
    } in cli_variants
    {
        // Find the `impl` block for the `builder` type of the same name.
        let impl_ = builder_content
            .iter()
            .filter_map(|i| match i {
                Item::Impl(ItemImpl {
                    self_ty,
                    trait_: None,
                    items,
                    ..
                }) => {
                    let Type::Path(TypePath { path, .. }) = &**self_ty else {
                        return None;
                    };
                    let struct_ident = path.segments.first().unwrap();
                    if struct_ident.ident != *variant_ident {
                        return None;
                    }
                    Some(items)
                }
                _ => None,
            })
            .next()
            .unwrap();

        // Get the return type for `send()`.
        let ret_type = impl_
            .iter()
            .filter_map(|i| match i {
                ImplItem::Fn(ImplItemFn {
                    sig:
                        Signature {
                            ident: fn_ident,
                            output: ReturnType::Type(_, ret),
                            ..
                        },
                    ..
                }) if fn_ident == "send" => Some(ret),
                _ => None,
            })
            .next()
            .unwrap();

        let Type::Path(TypePath { path, .. }) = &**ret_type else {
            unreachable!();
        };

        // Get the generic arguments to `Result<T>`.
        let last_segment = path.segments.last().unwrap();
        let PathArguments::AngleBracketed(args) = &last_segment.arguments else {
            unreachable!();
        };

        // Get the first type argument in `Result<T>`.
        let Some(GenericArgument::Type(Type::Path(type_path))) = args.args.first() else {
            unreachable!();
        };

        // This type is expected to always be a `ResponseValue<T>`.
        let segment = type_path.path.segments.last().unwrap();
        if segment.ident != "ResponseValue" {
            panic!("unexpected return type");
        }

        // Get its generic argument.
        let PathArguments::AngleBracketed(inner_args) = &segment.arguments else {
            unreachable!();
        };
        let Some(GenericArgument::Type(inner_type)) = inner_args.args.first() else {
            unreachable!();
        };

        // Skip variants that return `()`. This will cause the `--format` flag not be set for the
        // corresponding CLI subcommand.
        if matches!(inner_type, Type::Tuple(TypeTuple { elems, .. }) if elems.is_empty()) {
            continue;
        }

        let mut return_type = inner_type.clone();
        if let Type::Path(TypePath { path, .. }) = &mut return_type {
            let path_str = path
                .segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");

            // Skip any variant with a return type that cannot be reasonably presented in table
            // format.
            if is_unpresentable(&path_str) {
                continue;
            }

            // We need to convert the type name from declaration syntax to expression, e.g.,
            // `Vec<T>` to `Vec::<T>`.
            for segment in &mut path.segments {
                if let PathArguments::AngleBracketed(args) = &mut segment.arguments {
                    // Add the `::` token for turbofish syntax.
                    args.colon2_token = Some(syn::token::PathSep::default());
                }
            }
        }
        match_arms.push(quote! {
            CliCommand::#variant_ident => #return_type::field_names(),
        });
    }

    quote! {
        impl CliCommand {
            pub fn field_names(&self) -> &'static [&'static str] {
                match self {
                    #(#match_arms)*
                    _ => &[],
                }
            }
        }
    }
}

/// Generate a `ResponseFields` implementation for all types in the `oxide::types` module.
fn gen_response_fields_impls(sdk_file: &syn::File) -> TokenStream {
    let types_mod = sdk_file
        .items
        .iter()
        .filter_map(|i| match i {
            Item::Mod(module) => Some(module),
            _ => None,
        })
        .find(|m| m.ident == "types")
        .unwrap();

    let Some((_, types_content)) = &types_mod.content else {
        unreachable!();
    };

    let impls: TokenStream = types_content
        .iter()
        .filter(|i| matches!(i, Item::Enum(_) | Item::Struct(_)))
        .filter_map(|i| match i {
            Item::Enum(e) => {
                let (impl_generics, ty_generics, where_clause) = e.generics.split_for_impl();
                let impls = generate_enum_impl(e);
                let ident = &e.ident;

                Some(quote! {
                    impl #impl_generics ResponseFields for types::#ident #ty_generics #where_clause {
                        #impls
                    }
                })
            }
            Item::Struct(ItemStruct {
                fields,
                ident,
                generics,
                ..
            }) => {
                let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

               // For `ResultsPage` structs use the type they wrap.
               if ident.to_string().ends_with("ResultsPage") {
                   let ty = results_page_child_type(fields);

                   Some(quote! {
                    impl #impl_generics ResponseFields for types::#ident #ty_generics #where_clause {
                       type Item = types::#ty;

                       fn field_names() -> &'static [&'static str] {
                           types::#ty::field_names()
                       }

                       fn get_field(&self, _field_name: &str) -> Option<serde_json::Value> {
                           None
                       }

                        fn items(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
                            Box::new(self.items.iter())
                        }
                    }
                   })
               } else {
                let impls = generate_struct_impl(fields);
                Some(quote! {
                    impl #impl_generics ResponseFields for types::#ident #ty_generics #where_clause {
                        #impls
                    }
                })
               }
            }
            _ => None,
        })
        .collect();

    quote! {
        /// A trait for flexibly accessing objects returned by the Oxide API.
        pub trait ResponseFields {
            /// The individual object type returned from the API.
            ///
            /// Generally this is the type wrapped by a `ResponseValue`, but in some cases
            /// this is a collection of items. For example,
            /// `/v1/system/networking/bgp-routes-ipv4` returns a
            /// `ResponseValue<Vec<types::BgpImportedRouteIpv4>>`.
            type Item: ResponseFields;

            /// Get the field names of the object.
            ///
            /// For enums, the variant is included as "type".
            /// Unnamed fields are accessed as "value" for a tuple of arity 1, or "value_N", for
            /// larger arities.
            fn field_names() -> &'static [&'static str];

            /// Attempt to retrieve the specified field of an object as a JSON value.
            ///
            /// We convert to JSON instead of a string to provide callers with more flexibility
            /// in how they format the object.
            fn get_field(&self, field_name: &str) -> Option<serde_json::Value>;

            /// Get an iterator over all `Item`s contained in the response.
            fn items(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_>;
        }

        impl<T: ResponseFields> ResponseFields for Vec<T> {
            type Item = T;

            fn field_names() -> &'static [&'static str] {
                T::field_names()
            }

            fn get_field(&self, _field_name: &str) -> Option<serde_json::Value> {
                None
            }

            fn items(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
                Box::new(self.iter())
            }
        }

        #impls
    }
}

fn generate_struct_impl(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(fields) => {
            let field_names: Vec<_> = fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect();

            let field_accessors: Vec<_> = field_names
                .iter()
                .map(|field_name| {
                    let field_str = field_name.to_string();
                    quote! {
                        #field_str => serde_json::to_value(&self.#field_name).ok()
                    }
                })
                .collect();

            let field_strings: Vec<_> = field_names.iter().map(|ident| ident.to_string()).collect();

            quote! {
                type Item = Self;

                fn field_names() -> &'static [&'static str] {
                    &[#(#field_strings),*]
                }

                fn get_field(&self, field_name: &str) -> Option<serde_json::Value> {
                    match field_name {
                        #(#field_accessors,)*
                        _ => None,
                    }
                }

                fn items(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
                    Box::new(std::iter::once(self))
                }
            }
        }
        Fields::Unnamed(fields) => {
            let count = fields.unnamed.len();
            let (field_strings, field_accessors) = if count == 1 {
                let field_names = vec!["value".to_string()];

                // All single-value tuple structs implement `Deref` to access their payload.
                let field_accessors = vec![quote! {
                    "value" => serde_json::to_value(&*self).ok()
                }];
                (field_names, field_accessors)
            } else {
                let field_strings: Vec<_> = (0..count).map(|i| format!("value_{}", i)).collect();

                let field_accessors: Vec<_> = (0..count)
                    .map(|i| {
                        let index = Index::from(i);
                        let field_str = format!("value_{}", i);
                        quote! {
                            #field_str => serde_json::to_value(&self.#index).ok()
                        }
                    })
                    .collect();

                (field_strings, field_accessors)
            };

            quote! {
                type Item = Self;

                fn field_names() -> &'static [&'static str] {
                    &[#(#field_strings),*]
                }

                fn get_field(&self, field_name: &str) -> Option<serde_json::Value> {
                    match field_name {
                        #(#field_accessors,)*
                        _ => None,
                    }
                }

                fn items(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
                    Box::new(std::iter::once(self))
                }
            }
        }
        // Do not generate a `ResponseFields` impl for Unit structs, nothing to show.
        Fields::Unit => {
            quote! {}
        }
    }
}

fn generate_enum_impl(data: &ItemEnum) -> TokenStream {
    let enum_ident = &data.ident;

    // Collect all unique field names across all variants, including the variant name as "type".
    let mut field_strings = IndexSet::from(["type".to_string()]);

    for variant in &data.variants {
        match &variant.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    field_strings.insert(field.ident.as_ref().unwrap().to_string());
                }
            }
            Fields::Unnamed(fields) => {
                let count = fields.unnamed.len();
                if count == 1 {
                    field_strings.insert("value".to_string());
                } else {
                    for i in 0..count {
                        field_strings.insert(format!("value_{}", i));
                    }
                }
            }
            Fields::Unit => {}
        }
    }

    let field_strings: Vec<_> = field_strings.into_iter().collect();

    let variant_arms: Vec<_> = data
        .variants
        .iter()
        .map(|variant| {
            let variant_ident = &variant.ident;
            let variant_string = variant_ident.to_string();

            match &variant.fields {
                Fields::Named(fields) => {
                    let field_idents: Vec<_> = fields
                        .named
                        .iter()
                        .map(|f| f.ident.as_ref().unwrap()).collect();

                    let field_matches: Vec<_> = field_idents.iter().map(|field_ident| {
                            let field_string = field_ident.to_string();
                            quote! {
                                #field_string => serde_json::to_value(#field_ident).ok()
                            }
                        })
                        .collect();

                    quote! {
                        types::#enum_ident::#variant_ident { #(#field_idents),* } => {
                            match field_name {
                                "type" => Some(serde_json::Value::String(String::from(#variant_string))),
                                #(#field_matches,)*
                                _ => None,
                            }
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let count = fields.unnamed.len();

                    if count == 1 {
                        quote! {
                            types::#enum_ident::#variant_ident(value) => {
                                match field_name {
                                    "value" => serde_json::to_value(&value).ok(),
                                    _ => None,
                                }
                            }
                        }
                    } else {
                        let field_bindings: Vec<_> = (0..count)
                            .map(|i| {
                                syn::Ident::new(
                                    &format!("field_{}", i),
                                    proc_macro2::Span::call_site(),
                                )
                            })
                            .collect();

                        let field_matches: Vec<_> = field_bindings.iter().enumerate()
                            .map(|(i, ident)| {
                                let field_string = format!("value_{}", i);
                                quote! {
                                    #field_string => serde_json::to_value(#ident).ok()
                                }
                            })
                            .collect();

                        quote! {
                            #enum_ident::#variant_ident(#(#field_bindings),*) => {
                                match field_name {
                                    "type" => Some(serde_json::Value::String(String::from(#variant_string))),
                                    #(#field_matches,)*
                                    _ => None,
                                }
                            }
                        }
                    }
                }
                Fields::Unit => {
                    quote! {
                        types::#enum_ident::#variant_ident => {
                            match field_name {
                                "type" => Some(serde_json::Value::String(String::from(#variant_string))),
                                _ => None,
                            }
                        }
                    }
                }
            }
        })
        .collect();

    quote! {
        type Item = Self;

        fn field_names() -> &'static [&'static str] {
            &[#(#field_strings),*]
        }

        fn get_field(&self, field_name: &str) -> Option<serde_json::Value> {
            match self {
                #(#variant_arms,)*
            }
        }

        fn items(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
            Box::new(std::iter::once(self))
        }
    }
}

/// Extract the inner `T` from a `ResultsPage<T>`.
fn results_page_child_type(fields: &Fields) -> Type {
    let Fields::Named(named) = fields else {
        panic!("no named fields on ResultsPage");
    };

    // Find the `items` field on the `ResultsPage`.
    let items = named
        .named
        .iter()
        .filter(|&f| f.ident.as_ref().map(|ident| ident.to_string()) == Some(String::from("items")))
        .next()
        .unwrap();

    match &items.ty {
        Type::Path(TypePath { path, .. }) if path.segments.last().unwrap().ident == "Vec" => {
            // Access the generic arguments to `Vec<T>`.
            let PathArguments::AngleBracketed(args) = &path.segments.last().unwrap().arguments
            else {
                panic!("no angle bracket path on Vec");
            };

            // Take the inner `T`.
            let GenericArgument::Type(inner_type) = args.args.first().unwrap() else {
                panic!("no generic argument to Vec");
            };
            inner_type.clone()
        }
        _ => unreachable!(),
    }
}
