use rand::prelude::Distribution;
use rand::Rng;
use rand_regex::Regex;
use regex_syntax;
use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Debug, Formatter};

/// A valid regular expression that can be sampled for strings
/// which match the pattern.
///
/// # Examples
///
/// ```
/// use data_gen_lib::regex_pattern::RegexPattern;
/// use std::convert::TryInto;
///
/// let pattern: Result<RegexPattern, String> = r"\d{3}".try_into();
/// assert!(pattern.is_ok());
///
/// let invalid: Result<RegexPattern, String> = r"\d{3".try_into();
/// assert!(invalid.is_err())
/// ```
#[derive(Clone)]
pub struct RegexPattern {
    regex: Regex,
    format: String,
}

impl PartialEq for RegexPattern {
    fn eq(&self, other: &Self) -> bool {
        self.format.eq(&other.format)
    }
}

impl Eq for RegexPattern {}

impl Debug for RegexPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("{")?;
        f.write_str(self.format.as_str())?;
        f.write_str("}")
    }
}

impl Distribution<String> for RegexPattern {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        rng.sample(&self.regex)
    }
}

impl TryFrom<&str> for RegexPattern {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();

        parser
            .parse(value)
            .map_err(rand_regex::Error::Syntax)
            .and_then(move |hir| rand_regex::Regex::with_hir(hir, 100))
            .map(move |regex| RegexPattern {
                regex,
                format: value.into(),
            })
            .map_err(move |err| format!("invalid regular expression: {}", err))
    }
}

struct RegexPatternVisitor;

impl<'de> Visitor<'de> for RegexPatternVisitor {
    type Value = RegexPattern;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a valid regular expression")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        v.try_into().map_err(move |err| E::custom(err))
    }
}

impl<'de> Deserialize<'de> for RegexPattern {
    fn deserialize<D>(deserializer: D) -> Result<RegexPattern, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RegexPatternVisitor)
    }
}

impl Serialize for RegexPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.format.as_str())
    }
}
