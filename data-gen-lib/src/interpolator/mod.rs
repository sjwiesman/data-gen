use crate::interpolator::dataset::{get_dataset, DataSet};

mod address;
mod ancient;
mod dataset;

use nom::bytes::complete::{is_not, tag, take_until};

use nom::multi::many0;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use rand::prelude::Distribution;
use rand::prelude::*;
use rand::Rng;
use serde_json::Value;
use std::fmt::Formatter;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Interpolator {
    line: String,

    components: Vec<&'static DataSet>,
}

impl TryFrom<&str> for Interpolator {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let components = Interpolator::parser(value)
            .into_iter()
            .map(get_dataset)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| format!("invalid interpolation string: {}", err))?;

        Ok(Interpolator {
            line: value.to_string(),
            components,
        })
    }
}

struct InterpolatorVisitor;

impl<'de> Visitor<'de> for InterpolatorVisitor {
    type Value = Interpolator;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("invalid interpolation string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_string(v.to_string())
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let components = Interpolator::parser(&v)
            .into_iter()
            .map(get_dataset)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| serde::de::Error::custom(err))?;

        Ok(Interpolator {
            line: v,
            components,
        })
    }
}

impl<'de> Deserialize<'de> for Interpolator {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(InterpolatorVisitor)
    }
}

impl Serialize for Interpolator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.line)
    }
}

impl Distribution<Value> for Interpolator {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Value {
        let mut line = self.line.clone();
        for dataset in self.components.iter() {
            let replacement = dataset.options.choose(rng).unwrap_or(&"");
            line = line.replacen(dataset.tag, replacement, 1);
        }

        Value::String(line)
    }
}

impl Interpolator {
    const OPEN_TAG: &'static str = "#{";

    const CLOSE_TAG: &'static str = "}";

    fn parser(input: &str) -> Vec<&str> {
        let result: IResult<&str, Vec<&str>> = many0(preceded(
            take_until(Interpolator::OPEN_TAG),
            delimited(
                tag(Interpolator::OPEN_TAG),
                is_not(Interpolator::CLOSE_TAG),
                tag(Interpolator::CLOSE_TAG),
            ),
        ))(input);

        let (_, components) = result.expect("parsing an interpolator should never fail");
        components
    }
}

#[cfg(test)]
mod tests {
    use crate::interpolator::dataset::DataSet;
    use crate::interpolator::Interpolator;
    use lazy_static::lazy_static;
    use rand::{thread_rng, Rng};

    #[test]
    fn test_parser() {
        let components = Interpolator::parser("#{first-name}-#{last-name}");
        assert!(components.contains(&"first-name"));
        assert!(components.contains(&"last-name"))
    }

    lazy_static! {
        static ref TEST_DATA_SET: DataSet = DataSet {
            tag: "#{tag}",
            options: vec!["replace"]
        };
    }

    #[test]
    fn test_interpolator() {
        let interpolator: Interpolator = Interpolator {
            line: "#{tag}".to_string(),
            components: vec![&TEST_DATA_SET],
        };

        let result = thread_rng().sample(interpolator);
        assert_eq!(result, "replace")
    }
}
