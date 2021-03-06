use crate::data_type::DataType;
use crate::interpolator::Interpolator;
use crate::physical::distributions::{DynDistribution, Static, Supplier};
use crate::regex_pattern::RegexPattern;
use chrono::Local;
use rand::distributions::Distribution;
use rand::prelude::*;
use serde_json::{json, Number, Value};
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::{Deref, Range};

use super::distributions::Iter;

/// The physical representation of a [DataType], this enum
/// defines how fields are generated. Many different logical
/// [DataType]'s may map to the same [PhysicalDataType].
pub enum PhysicalDataType<'a> {
    Array {
        element: Box<PhysicalDataType<'a>>,
        size: i32,
    },
    Boolean,
    Generator {
        format: Interpolator,
    },
    OneOf {
        options: Vec<&'a str>,
    },
    Proxy {
        f: Box<dyn DynDistribution>,
    },
    Range {
        r: Range<i64>,
    },
    Regex {
        pattern: RegexPattern,
    },
    Object {
        fields: HashMap<&'a str, PhysicalDataType<'a>>,
    },
}

impl<'a> From<&DataType<'a>> for PhysicalDataType<'a> {
    fn from(dt: &DataType<'a>) -> Self {
        match dt {
            DataType::Array { element, size } => PhysicalDataType::Array {
                element: Box::new(element.deref().into()),
                size: *size as i32,
            },
            DataType::Boolean => PhysicalDataType::Boolean,
            DataType::Literal { value } => PhysicalDataType::Proxy {
                f: Box::new(Static::new(*value)),
            },
            DataType::OneOf { options } => PhysicalDataType::OneOf {
                options: options.clone(),
            },
            DataType::PhoneNumber => PhysicalDataType::Regex {
                pattern: r"\d{3}-\d{3}-\d{4}".to_owned().try_into().unwrap(),
            },
            DataType::SmallInt => PhysicalDataType::Range { r: -32768..32768 },
            DataType::Integer => PhysicalDataType::Range {
                r: -2147483648..2147483648,
            },
            DataType::Range { from, to } => PhysicalDataType::Range { r: *from..*to },
            DataType::Regex { pattern } => PhysicalDataType::Regex {
                pattern: pattern.clone(),
            },
            DataType::Generator { format } => PhysicalDataType::Generator {
                format: format.clone(),
            },
            DataType::Object { fields } => PhysicalDataType::Object {
                fields: fields.iter().map(|(name, dt)| (*name, dt.into())).collect(),
            },
            DataType::Serial => PhysicalDataType::Proxy {
                f: Box::new(Iter::new((1..=2147483647).map(|id| json!(id)))),
            },
            DataType::Timestamp => PhysicalDataType::Proxy {
                f: Box::new(Supplier::new(timestamp)),
            },
        }
    }
}

fn timestamp() -> Value {
    Value::String(Local::now().format("%F %r").to_string())
}

impl Distribution<Value> for PhysicalDataType<'_> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Value {
        match self {
            PhysicalDataType::Array { element, size } => {
                let length: i32 = rng.gen_range(1..=*size);
                let elements = (0..length)
                    .into_iter()
                    .map(move |_| element.sample(rng))
                    .collect();

                Value::Array(elements)
            }
            PhysicalDataType::Boolean => Value::Bool(rng.gen()),
            PhysicalDataType::Generator { format } => Value::String(format.sample(rng)),
            PhysicalDataType::Proxy { f } => f.sample(rng),
            PhysicalDataType::Regex { pattern } => Value::String(pattern.sample(rng)),
            PhysicalDataType::Range { ref r } => {
                Value::Number(Number::from(rng.gen_range(r.clone())))
            }
            PhysicalDataType::OneOf { options } => match options.choose(rng) {
                None => Value::Null,
                Some(choice) => Value::String(choice.to_string()),
            },
            PhysicalDataType::Object { fields } => {
                let components = fields
                    .iter()
                    .map(move |(name, dt)| (name.to_string(), dt.sample(rng)))
                    .collect();

                Value::Object(components)
            }
        }
    }
}
