// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

//! A Facility to generate valid, random data for types that are
//! `schemars::JsonSchema + serde::Deserialize`. It generates a JSON value
//! based on the JSON schema and then deserializes that into the given type.

use std::fmt::Display;

use chrono::{NaiveDate, NaiveTime, Utc};
use rand::Rng;
use schemars::{
    schema::{
        ArrayValidation, InstanceType, NumberValidation, ObjectValidation, Schema, SchemaObject,
        SingleOrVec, StringValidation, SubschemaValidation,
    },
    schema_for, JsonSchema, Map,
};
use serde::de::DeserializeOwned;
use serde_json::{json, Number, Value};

/// Notes
/// Configuration we might want
/// - Should we use examples if they're present? Seems like might be a enum
///   to select between "ignore examples", "exclusively use examples",
///   "choose between generated values and examples (with some weight)",
///   "choose examples for scalars (but not complex types)", etc.
/// - One can imagine situations where it may not be tractable to know a priori
///   if a value validates, for example if there were a structure that
///   contained a complex regex or a `not` with a complex schema. The consumer
///   might want to decide how many times we try to generate a type before
///   giving up with an error.
/// - Maximum array length.
///
///
/// Do we need something to ensure that recursive types converge? For example,
/// we could track the depth of an object (per-reference or in absolute terms)
/// and bias towards simpler types as the depth increases (e.g. preferring to
/// exclude non-required object properties or shortening arrays).

// TODO This is going to let us have a fixed set of bytes or an RNG.
pub trait Source: Rng {}
impl<T> Source for T where T: Rng {}

#[derive(Debug)]
pub struct Error {}
impl Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
impl std::error::Error for Error {}

pub trait JsonMock: Sized {
    fn mock_value(src: &mut impl Source) -> Result<Self, Error>;
}
impl<T> JsonMock for T
where
    T: JsonSchema + DeserializeOwned + Sized,
{
    fn mock_value(src: &mut impl Source) -> Result<Self, Error> {
        let root = schema_for!(Self);

        let value = mock_schema_object(&root.schema, &root.definitions, src)?;

        let Ok(instance) = serde_json::from_value(value.clone()) else {
            panic!(
                "{}\nvalue:\n{}\nschema:\n{}",
                "the generated value could not be serialized; \
                likely, the generated value omitted some constraint, but it's \
                possible that the JsonSchema and Serialize impls diverge.",
                serde_json::to_string_pretty(&value).unwrap_or("<error>".to_string()),
                serde_json::to_string_pretty(&root).unwrap_or("<error>".to_string()),
            )
        };

        Ok(instance)
    }
}

fn mock_schema(
    schema: &Schema,
    definitions: &Map<String, Schema>,
    src: &mut impl Source,
) -> Result<Value, Error> {
    match schema {
        Schema::Bool(false) => unimplemented!(),
        Schema::Bool(true) => mock_schema_any(src),
        Schema::Object(obj) => mock_schema_object(obj, definitions, src),
    }
}

fn mock_schema_any(_src: &mut impl Source) -> Result<Value, Error> {
    Ok(json!({ "potatoes" : 42 }))
}

fn mock_schema_object(
    schema: &SchemaObject,
    definitions: &Map<String, Schema>,
    src: &mut impl Source,
) -> Result<Value, Error> {
    match schema {
        // Enumerated values
        SchemaObject {
            enum_values: Some(values),
            ..
        } => Ok(values[src.gen_range(0..values.len())].clone()),

        // A constant value
        SchemaObject {
            const_value: Some(value),
            ..
        } => Ok(value.clone()),

        // Subschemas
        SchemaObject {
            metadata: _,
            instance_type: None,
            format: None,
            enum_values: None,
            const_value: None,
            subschemas: Some(subschemas),
            number: None,
            string: None,
            array: None,
            object: None,
            reference: None,
            extensions: _,
        } => mock_subschemas(subschemas, definitions, src),

        SchemaObject {
            metadata: _,
            instance_type: Some(SingleOrVec::Single(instance_type)),
            format,
            enum_values: None,
            const_value: None,
            subschemas: None,
            number,
            string,
            array,
            object,
            reference: None,
            extensions: _,
        } => match instance_type.as_ref() {
            InstanceType::Null => Ok(json! { null }),
            InstanceType::Boolean => mock_bool(src),
            InstanceType::Object => mock_object(object, definitions, src),
            InstanceType::Array => mock_array(array, definitions, src),
            InstanceType::Number => mock_number(number, src),
            InstanceType::String => mock_string(format, string, src),
            InstanceType::Integer => mock_integer(format, number, src),
        },

        // One way to express an option
        SchemaObject {
            metadata: _,
            instance_type: Some(SingleOrVec::Vec(vec)),
            format,
            enum_values: None,
            const_value: None,
            subschemas: None,
            number: _,
            string,
            array: _,
            object: _,
            reference: None,
            extensions: _,
        } if matches!(&vec[..], &[InstanceType::String, InstanceType::Null])
            | matches!(&vec[..], &[InstanceType::Null, InstanceType::String]) =>
        {
            mock_string(format, string, src)
        }

        SchemaObject {
            metadata: _,
            instance_type: None,
            format: None,
            enum_values: None,
            const_value: None,
            subschemas: None,
            number: None,
            string: None,
            array: None,
            object: None,
            reference: Some(ref_name),
            extensions: _,
        } => {
            let Some(ii) = ref_name.rfind('/') else {
                panic!("invalid reference name {}", ref_name);
            };

            let Some(schema) = definitions.get(&ref_name[ii + 1..]) else {
                panic!();
            };

            mock_schema(schema, definitions, src)
        }

        s => todo!("schema {}", serde_json::to_string_pretty(s).unwrap()),
    }
}

fn mock_subschemas(
    subschemas: &SubschemaValidation,
    definitions: &std::collections::BTreeMap<String, Schema>,
    src: &mut impl Source,
) -> Result<Value, Error> {
    match subschemas {
        // Single subschema; this is typical of an outer schema for additional
        // metadata
        SubschemaValidation {
            all_of: Some(subs),
            any_of: None,
            one_of: None,
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        }
        | SubschemaValidation {
            all_of: None,
            any_of: Some(subs),
            one_of: None,
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        } if subs.len() == 1 => mock_schema(subs.first().unwrap(), definitions, src),

        // allOf
        SubschemaValidation {
            all_of: Some(_),
            any_of: None,
            one_of: None,
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        } => {
            // TODO To properly handle this, we need to some mechanism for for
            // taking the union of all constraints. I think the best way to do
            // this is going to be by creating a "merged schema" whereby we
            // cram schemas together. This gets even more tricky when
            // subschemas themselves contain `one_of` or `any_of`
            // constructions. Perhaps we want to resolve these into some sort
            // of disjunctive normal form where we resolve to a `one_of`.
            todo!("tricky to handle")
        }

        // anyOf
        SubschemaValidation {
            all_of: None,
            any_of: Some(any_of),
            one_of: None,
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        } => {
            // TODO What would this look like ideally? Effectively, we want to
            // consider all permutations of subschemas and then prune the ones
            // that are intrinsically invalid i.e. the ones that contain
            // mutually incompatible schemas. We can think of this as producing
            // a `one_of` consisting of each of those valid permutations.
            //
            // This is pretty tricky, so for the moment we'll prune all
            // permutations with more than one schema--which is a fancy way of
            // saying that we'll just use one schema at a time as with `one_of`
            // below.
            mock_schema(&any_of[src.gen_range(0..any_of.len())], definitions, src)
        }

        // oneOf
        SubschemaValidation {
            all_of: None,
            any_of: None,
            one_of: Some(one_of),
            not: None,
            if_schema: None,
            then_schema: None,
            else_schema: None,
        } => mock_schema(&one_of[src.gen_range(0..one_of.len())], definitions, src),

        // if-then-else
        SubschemaValidation {
            all_of: None,
            any_of: None,
            one_of: None,
            not: None,
            if_schema: Some(_),
            then_schema: Some(then_schema),
            else_schema: Some(else_schema),
        } => {
            // This is fun! We can just roll the dice and pick between them!
            if src.gen() {
                mock_schema(then_schema, definitions, src)
            } else {
                mock_schema(else_schema, definitions, src)
            }
        }
        _ => todo!(),
    }
}

/// To generate a floating point number, we don't really need to worry about
/// the format since it's all going to end up as a Value. We can just proceed
/// with f64 and it will all work out.
fn mock_number(
    number: &Option<Box<NumberValidation>>,
    src: &mut impl Source,
) -> Result<Value, Error> {
    let NumberValidation {
        multiple_of,
        maximum,
        exclusive_maximum,
        minimum,
        exclusive_minimum,
    } = number.as_deref().cloned().unwrap_or_default();

    let min = match (minimum, exclusive_minimum) {
        (None, None) => None,
        (None, Some(v)) => Some(v),
        (Some(v), None) => Some(v),
        (Some(v1), Some(v2)) => Some(v1.max(v2)),
    };

    let max = match (maximum, exclusive_maximum) {
        (None, None) => None,
        (None, Some(v)) => Some(v),
        (Some(v), None) => Some(v),
        (Some(v1), Some(v2)) => Some(v1.min(v2)),
    };

    let mult = match multiple_of {
        Some(m) => m,
        _ => 1.0,
    };

    let value = match (min, max) {
        (None, None) => src.gen::<f64>() * mult,
        (None, Some(max)) => src.gen_range(f64::MIN..max / mult) * mult,
        (Some(min), None) => src.gen_range(min / mult..f64::MAX) * mult,
        (Some(min), Some(max)) => src.gen_range(min / mult..max / mult) * mult,
    };

    let Some(value) = Number::from_f64(value) else {
        unreachable!("error with the generated value {}", value);
    };

    Ok(Value::Number(value))
}

macro_rules! mock_int_type {
    ($number:expr, $int_type:ty, $src:expr) => {{
        let NumberValidation {
            multiple_of,
            maximum,
            exclusive_maximum,
            minimum,
            exclusive_minimum,
        } = $number.as_deref().cloned().unwrap_or_default();
        let (min, max) = (<$int_type>::MIN, <$int_type>::MAX);
        let min = match exclusive_minimum {
            Some(m) if m.fract().abs() > f64::EPSILON => {
                panic!("{} is not close enough to being an integer", m)
            }
            Some(m) if (m + 1.0) as $int_type > min => (m + 1.0) as $int_type,
            _ => min,
        };
        let min = match minimum {
            Some(m) if m.fract().abs() > f64::EPSILON => {
                panic!("{} is not close enough to being an integer", m)
            }
            Some(m) if m as $int_type > min => m as $int_type,
            _ => min,
        };
        let max = match exclusive_maximum {
            Some(m) if m.fract().abs() > f64::EPSILON => {
                panic!("{} is not close enough to being an integer", m)
            }
            Some(m) if max > (m - 1.0) as $int_type => (m - 1.0) as $int_type,
            _ => max,
        };
        let max = match maximum {
            Some(m) if m.fract().abs() > f64::EPSILON => {
                panic!("{} is not close enough to being an integer", m)
            }
            Some(m) if max > m as $int_type => m as $int_type,
            _ => max,
        };

        match multiple_of {
            Some(m) if m.fract().abs() > f64::EPSILON => {
                panic!("{} is not close enough to being an integer", m)
            }
            Some(m) => {
                let mult = m as $int_type;
                let min = (min + mult - 1) / mult;
                let max = max / mult;

                let value = $src.gen_range(min..=max) + mult;
                Ok(Value::Number(Number::from(value)))
            }
            None => {
                let value = $src.gen_range(min..=max);
                Ok(Value::Number(Number::from(value)))
            }
        }
    }};
}

fn mock_bool(src: &mut impl Source) -> Result<Value, Error> {
    let b = src.gen_bool(0.5);
    Ok(json! { b })
}

fn mock_integer(
    format: &Option<String>,
    number: &Option<Box<NumberValidation>>,
    src: &mut impl Source,
) -> Result<Value, Error> {
    match format.as_deref() {
        Some("uint8") => mock_int_type!(number, u8, src),
        Some("int8") => mock_int_type!(number, i8, src),
        Some("uint16") => mock_int_type!(number, u16, src),
        Some("int16") => mock_int_type!(number, i16, src),
        Some("uint32") => mock_int_type!(number, u32, src),
        Some("int32") => mock_int_type!(number, i32, src),
        Some("uint64") => mock_int_type!(number, u64, src),
        None | Some("int64") => mock_int_type!(number, i64, src),
        Some(x) => panic!("unsupported integer format: {}", x),
    }
}

fn mock_string(
    format: &Option<String>,
    string: &Option<Box<StringValidation>>,
    src: &mut impl Source,
) -> Result<Value, Error> {
    // TODO use the validation fields
    let StringValidation {
        max_length: _,
        min_length: _,
        pattern: _,
    } = string.as_deref().cloned().unwrap_or_default();

    match format.as_deref() {
        Some("uuid") => {
            let data: [u8; 16] = src.gen();
            let uuid = uuid::Uuid::from_slice(&data).unwrap();
            return Ok(serde_json::to_value(uuid).unwrap());
        }
        Some("date-time") => {
            let dt = NaiveDate::from_num_days_from_ce_opt(src.gen_range(710_000..740_000))
                .unwrap()
                .and_time(
                    NaiveTime::from_num_seconds_from_midnight_opt(
                        src.gen_range(0..24 * 60 * 60),
                        0,
                    )
                    .unwrap(),
                )
                .and_local_timezone(Utc)
                .unwrap();

            return Ok(serde_json::to_value(dt).unwrap());
        }
        _ => (),
    }

    // TODO
    // Dealing with the regex is hard; some folks have tackled that... but the
    // regexes we have in omicron (e.g. Name) are very complex. We have a few
    // options:
    // 1. ignore them and just hope we don't conflict
    // 2. generate strings and try again N times if the string doesn't validate
    // 3. use examples and make sure we have at least a couple of them.
    //
    // Absent regexes, I think I'll..
    // 1. Pick some "reasonable" string length
    // 2. Concatenate strings until we get in some range
    // 3. Use the TWL as a string source
    // 4. Randomly choose among e.g. kebab, snake, space, pascal, etc.

    const TWL_Q_QU: [&str; 30] = [
        "buqsha",
        "buqshas",
        "faqir",
        "faqirs",
        "qaid",
        "qaids",
        "qanat",
        "qanats",
        "qat",
        "qats",
        "qindar",
        "qindarka",
        "qindarkas",
        "qindars",
        "qintar",
        "qintars",
        "qiviut",
        "qiviuts",
        "qoph",
        "qophs",
        "qwerty",
        "qwertys",
        "sheqalim",
        "sheqel",
        "suq",
        "suqs",
        "tranq",
        "tranqs",
        "umiaq",
        "umiaqs",
    ];

    Ok(Value::String(
        TWL_Q_QU[src.gen_range(0..TWL_Q_QU.len())].to_string(),
    ))
}

fn mock_array(
    array: &Option<Box<ArrayValidation>>,
    definitions: &std::collections::BTreeMap<String, Schema>,
    src: &mut impl Source,
) -> Result<Value, Error> {
    let ArrayValidation {
        items,
        additional_items,
        max_items,
        min_items,
        unique_items,
        contains,
    } = array.as_deref().cloned().unwrap_or_default();

    // TODO here's how contains is defined:
    // 9.3.1.4.  contains
    //    The value of this keyword MUST be a valid JSON Schema.
    //
    //    An array instance is valid against "contains" if at least one of its
    //    elements is valid against the given schema.  Note that when
    //    collecting annotations, the subschema MUST be applied to every array
    //    element even after the first match has been found.  This is to ensure
    //    that all possible annotations are collected.
    //
    // How would we implement this? We'd need to figure out which of the item
    // schemas (or additional_items schema) might be compatible with the
    // contains schema, then choose one of those and generate some sort of
    // intersection schema and then generate a type from that.
    assert!(contains.is_none());

    // TODO this seems like a pain in the neck to handle... so we won't until
    // the need arises.
    assert!(unique_items.is_none() || unique_items == Some(false));

    let min = min_items.unwrap_or(0);
    // TODO some sort of config for the maximum array length?
    let max = match max_items {
        Some(n) if n < 5 => n,
        _ if min > 5 => min,
        _ => 5,
    };

    let len = src.gen_range(min..max) as usize;

    (0..len)
        .map(|ii| {
            let s = match &items {
                None => additional_items.as_deref(),
                Some(SingleOrVec::Single(s)) => Some(s.as_ref()),
                Some(SingleOrVec::Vec(v)) => v.get(ii).or(additional_items.as_deref()),
            };

            match s {
                Some(s) => mock_schema(s, definitions, src),
                None => mock_schema_any(src),
            }
        })
        .collect::<Result<Vec<_>, Error>>()
        .map(Value::Array)
}

fn mock_object(
    object: &Option<Box<ObjectValidation>>,
    definitions: &std::collections::BTreeMap<String, Schema>,
    src: &mut impl Source,
) -> Result<Value, Error> {
    let ObjectValidation {
        max_properties,
        min_properties,
        required,
        properties,
        pattern_properties,
        additional_properties: _,
        property_names,
    } = object.as_deref().cloned().unwrap_or_default();

    let mut value = serde_json::Map::new();

    assert!(max_properties.is_none());
    assert!(min_properties.is_none());
    assert!(pattern_properties.is_empty());
    assert!(property_names.is_none());

    let (required_properties, optional_properties): (Vec<_>, Vec<_>) = properties
        .iter()
        .partition(|(prop_name, _)| required.contains(*prop_name));

    for (prop_name, prop_schema) in required_properties {
        value.insert(
            prop_name.clone(),
            mock_schema(prop_schema, definitions, src)?,
        );
    }

    for (prop_name, prop_schema) in optional_properties {
        if src.gen() {
            value.insert(
                prop_name.clone(),
                mock_schema(prop_schema, definitions, src)?,
            );
        }
    }

    Ok(Value::Object(value))
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use schemars::JsonSchema;
    use serde::Deserialize;

    use super::JsonMock;

    #[test]
    fn test_one() {
        #[allow(dead_code)]
        #[derive(Deserialize, JsonSchema, Debug)]
        struct TestInner {
            a: u32,
            b: String,
        }
        #[allow(dead_code)]
        #[derive(Deserialize, JsonSchema, Debug)]
        struct Test {
            a: u32,
            b: String,
            t: Vec<TestInner>,
        }

        let mut src = rand::rngs::SmallRng::seed_from_u64(42);

        let value = Test::mock_value(&mut src);

        assert!(value.is_ok());
    }
}
