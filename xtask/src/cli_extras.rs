use indexmap::IndexSet;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use std::sync::OnceLock;
use syn::{
    Fields, GenericArgument, Generics, Ident, ImplItem, ImplItemFn, Index, Item, ItemEnum,
    ItemImpl, ItemStruct, Path, PathArguments, ReturnType, Signature, Type, TypePath, Variant,
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

/// Generate a `ResponseFields` implementation for all types in the `oxide::types` module and
/// inject it into `CliCommand`.
pub fn gen_response_fields(sdk_tokens: TokenStream, cli_tokens: TokenStream) -> TokenStream {
    let sdk_file: syn::File = syn::parse2(sdk_tokens).unwrap();
    let cli_file: syn::File = syn::parse2(cli_tokens).unwrap();

    let response_field_tokens = gen_response_fields_impls(&sdk_file);
    let cli_cmd_tokens = gen_cli_command(&sdk_file, &cli_file);

    quote! {

        #response_field_tokens

        #cli_cmd_tokens
    }
}

/// Generate `ResponseFields` impls for all types returned by a `send` fn.
pub fn gen_response_fields_impls(sdk_file: &syn::File) -> TokenStream {
    let builder_content = mod_content(sdk_file, "builder");
    let types_content = mod_content(sdk_file, "types");

    let mut implemented_types = HashSet::new();
    let mut impls = TokenStream::new();

    for impl_items in builder_content.iter().filter_map(|i| match i {
        Item::Impl(ItemImpl {
            trait_: None,
            items,
            ..
        }) => Some(items.as_slice()),
        _ => None,
    }) {
        let ret_type = send_return_type(impl_items);
        let inner_type = response_value_inner_type(ret_type);
        let Some(ident) = filter_innermost_type_ident(inner_type) else {
            continue;
        };
        if let Some(tokens) = gen_for_ident(types_content, &mut implemented_types, &ident) {
            impls.extend(tokens);
        }
    }

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

        impl ResponseFields for types::Error {
            type Item = Self;

            fn field_names() -> &'static [&'static str] {
                &[]
            }

            fn get_field(&self, _field_name: &str) -> Option<serde_json::Value> {
                None
            }

            fn items(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
                Box::new(std::iter::empty())
            }
        }

        #impls
    }
}

/// Extract the content of a module.
fn mod_content<'a>(file: &'a syn::File, name: &str) -> &'a Vec<Item> {
    let module = file
        .items
        .iter()
        .filter_map(|i| match i {
            Item::Mod(module) => Some(module),
            _ => None,
        })
        .find(|m| m.ident == name)
        .unwrap();

    let Some((_, content)) = &module.content else {
        unreachable!("no module content");
    };

    content
}

/// Get the return type for `send()`.
fn send_return_type(items: &[ImplItem]) -> Type {
    items
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
        .map(|ret_type| *ret_type.clone())
        .expect("no send fn in block")
}

/// Extract the inner `T` from `Result<ResponseValue<T>, E>`.
fn response_value_inner_type(ret_type: Type) -> Type {
    let Type::Path(TypePath { path, .. }) = ret_type else {
        unreachable!("type was not a path");
    };

    // Get the generic arguments to `Result<T>`.
    let last_segment = path.segments.last().unwrap();
    let PathArguments::AngleBracketed(args) = &last_segment.arguments else {
        unreachable!("no angle bracket on path");
    };

    // Get the first type argument in `Result<T>`.
    let Some(GenericArgument::Type(Type::Path(type_path))) = args.args.first() else {
        unreachable!("no generic args in path");
    };

    // This type must be a `ResponseValue<T>`.
    let segment = type_path.path.segments.last().unwrap();
    if segment.ident != "ResponseValue" {
        unreachable!("return type was not a ResponseValue");
    }

    // Get its generic argument.
    let PathArguments::AngleBracketed(inner_args) = &segment.arguments else {
        unreachable!("no angle bracket on ResponseValue");
    };
    let Some(GenericArgument::Type(inner_type)) = inner_args.args.first() else {
        unreachable!("no generic args in ResponseValue");
    };

    inner_type.clone()
}

/// Get the `Ident` of the type being returned, filtering for types that need an
/// implementation of `ResponseFields`.
fn filter_innermost_type_ident(ty: Type) -> Option<Ident> {
    // This filters out `()`.
    let Type::Path(TypePath { path, .. }) = ty else {
        return None;
    };

    let mut inner_ident = path.segments.last()?.ident.clone();
    let mut path_str = path_to_string(&path);

    if inner_ident == "Vec" {
        let PathArguments::AngleBracketed(args) = &path.segments.last()?.arguments else {
            return None;
        };

        // Take the inner `T`.
        let GenericArgument::Type(vec_ty) = args.args.first()? else {
            return None;
        };

        let Type::Path(TypePath { path: vec_path, .. }) = vec_ty else {
            return None;
        };
        inner_ident = vec_path.segments.last()?.ident.clone();
        path_str = path_to_string(vec_path);
    }

    // Only handle `oxide::types` types, we don't need impls for the others.
    if path_str.starts_with("types::") {
        Some(inner_ident)
    } else {
        None
    }
}

/// Generate a `ResponseFields` impl for an `Ident`.
fn gen_for_ident(
    types_content: &[Item],
    implemented_types: &mut HashSet<String>,
    ty_ident: &Ident,
) -> Option<TokenStream> {
    if !implemented_types.insert(ty_ident.to_string()) {
        return None;
    }

    match types_content
        .iter()
        .find(|i|
            matches!(i, Item::Enum(ItemEnum { ident, ..} ) | Item::Struct(ItemStruct { ident, .. }) if ident == ty_ident)
        ) {
            Some(Item::Enum(e)) => {
                let (impl_generics, ty_generics, where_clause) = e.generics.split_for_impl();
                let impls = gen_enum_impl(e);
                let ident = &e.ident;

                Some(quote! {
                    impl #impl_generics ResponseFields for types::#ident #ty_generics #where_clause {
                        #impls
                    }
                })
            }
            Some(Item::Struct(ItemStruct {
                fields,
                ident,
                generics,
                ..
            })) => {
               if ident.to_string().ends_with("ResultsPage") {
                   gen_result_page_impl(ident, fields, types_content, generics, implemented_types)
               } else {
                let impls = gen_struct_impl(fields);
                let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

                Some(quote! {
                    impl #impl_generics ResponseFields for types::#ident #ty_generics #where_clause {
                        #impls
                    }
                })
               }
            }
            _ => unreachable!("return type was neither a struct nor enum"),
        }
}

/// Generate a `ResponseFields` impl for a `ResultPage` type and its wrapped `items` type.
fn gen_result_page_impl(
    ident: &Ident,
    fields: &Fields,
    types_content: &[Item],
    generics: &Generics,
    implemented_types: &mut HashSet<String>,
) -> Option<TokenStream> {
    let ty = results_page_child_type(fields);
    let Type::Path(TypePath { path, .. }) = &ty else {
        return None;
    };

    let child_ident = path.segments.last().cloned().map(|s| s.ident)?;
    let child_impl = gen_for_ident(types_content, implemented_types, &child_ident);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    Some(quote! {
     #child_impl

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
}

/// Extract the inner `T` from a `ResultsPage<T>`.
fn results_page_child_type(fields: &Fields) -> Type {
    let Fields::Named(named) = fields else {
        unreachable!("no named fields on ResultsPage");
    };

    // Find the `items` field on the `ResultsPage`.
    let items = named
        .named
        .iter()
        .find(|&f| f.ident.as_ref().map(|ident| ident.to_string()) == Some(String::from("items")))
        .unwrap();

    match &items.ty {
        Type::Path(TypePath { path, .. }) if path.segments.last().unwrap().ident == "Vec" => {
            // Access the generic arguments to `Vec<T>`.
            let PathArguments::AngleBracketed(args) = &path.segments.last().unwrap().arguments
            else {
                unreachable!("no angle bracket path on Vec");
            };

            // Take the inner `T`.
            let GenericArgument::Type(inner_type) = args.args.first().unwrap() else {
                unreachable!("no generic argument to Vec");
            };
            inner_type.clone()
        }
        _ => unreachable!(),
    }
}

/// Generate a `ResponseFields` impl for a struct.
fn gen_struct_impl(fields: &Fields) -> TokenStream {
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

                #[allow(clippy::needless_borrows_for_generic_args)]
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

                #[allow(clippy::borrow_deref_ref)]
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

/// Generate a `ResponseFields` impl for an enum.
fn gen_enum_impl(data: &ItemEnum) -> TokenStream {
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

/// Generate a `field_names` method for `CliCommand` use to determine which subcommands
/// are eligible for the `--format` flag, and adding the list of available fields `--help`
/// output.
fn gen_cli_command(sdk_file: &syn::File, cli_file: &syn::File) -> TokenStream {
    let builder_content = mod_content(sdk_file, "builder");

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

    // We expect tabular formatting to be useful when querying. Creation APIs in particular already
    // have many arguments, adding the bulky `--format` help text on top of that will make them
    // harder to understand. However, there is no 100% reliable way to determine which subcommand is
    // performing a query. The return type for create and view endpoints are frequently the same, so
    // we can't discriminate by type. Trying to limit to `List` and `View` will miss a fair number
    // of query endpoints, e.g. `NetworkingBgpImportedRoutesIpv4` and `CurrentUserGroups`. The
    // heuristic below should catch most new write operations.
    let action_words = HashSet::from([
        "Add", "Attach", "Create", "Demote", "Detach", "Probe", "Promote", "Reboot", "Resend",
        "Start", "Stop", "Update",
    ]);

    let mut match_arms = Vec::new();
    for Variant {
        ident: variant_ident,
        ..
    } in cli_variants
    {
        let variant_str = variant_ident.to_string();
        let last_word = variant_str
            .rfind(char::is_uppercase)
            .map(|i| &variant_str[i..])
            .unwrap_or_default();

        if action_words.contains(last_word) {
            continue;
        }

        let impl_items = builder_type_for_variant(variant_ident, builder_content);
        let ret_type = send_return_type(impl_items);
        let inner_type = response_value_inner_type(ret_type);
        if let Some(return_path) = extract_type_path_expression(inner_type) {
            match_arms.push(quote! {
                CliCommand::#variant_ident => #return_path::field_names(),
            });
        }
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

/// Find the corresponding `impl` block in the `builder` mod for a `CliCommand` variant.
fn builder_type_for_variant<'a>(
    variant_ident: &Ident,
    builder_content: &'a [Item],
) -> &'a [ImplItem] {
    builder_content
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
                let struct_ident = path.segments.first()?;
                if &struct_ident.ident != variant_ident {
                    return None;
                }
                Some(items)
            }
            _ => None,
        })
        .map(|i| i.as_slice())
        .next()
        .expect("no corresponding builder type for CliCommand variant")
}

/// Convert a `Type` to a `Path` in expression format.
fn extract_type_path_expression(mut inner_ty: Type) -> Option<Path> {
    let Type::Path(TypePath { path, .. }) = &mut inner_ty else {
        return None;
    };
    let path_str = path_to_string(path);

    // Skip any variant with a return type that cannot be reasonably presented in table
    // format.
    if is_unpresentable(&path_str) {
        return None;
    }

    // We need to convert the type name from declaration syntax to expression, e.g.,
    // `Vec<T>` to `Vec::<T>`.
    for segment in &mut path.segments {
        if let PathArguments::AngleBracketed(args) = &mut segment.arguments {
            // Add the `::` token for turbofish syntax.
            args.colon2_token = Some(syn::token::PathSep::default());
        }
    }

    Some(path.clone())
}

fn path_to_string(path: &Path) -> String {
    path.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::")
}
