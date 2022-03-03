use crate::data_type::DataType;
use crate::interpolator::Interpolator;
use crate::physical::distributions::DynDistribution;
use crate::regex_pattern::RegexPattern;
use chrono::Local;
use rand::distributions::Distribution;
use rand::prelude::*;
use serde_json::{Number, Value};
use std::collections::HashMap;
use std::convert::TryInto;
use std::ops::{Deref, Range};

/// The physical representation of a [DataType], this enum
/// defines how fields are generated. Many different logical
/// [DataType]'s may map to the same [PhysicalDataType].
pub enum PhysicalDataType<'a> {
    Array {
        element: Box<PhysicalDataType<'a>>,
        size: u32,
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
        r: Range<i32>,
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
                size: *size,
            },
            DataType::Boolean => PhysicalDataType::Boolean,
            DataType::Literal { value } => PhysicalDataType::Proxy { f: (*value).into() },
            DataType::OneOf { options } => PhysicalDataType::OneOf {
                options: options.clone(),
            },
            DataType::PhoneNumber => PhysicalDataType::Regex {
                pattern: r"\d{3}-\d{3}-\d{4}".to_owned().try_into().unwrap(),
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
            DataType::Timestamp => PhysicalDataType::Proxy {
                f: timestamp.into(),
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
                let elements = (0..*size)
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
