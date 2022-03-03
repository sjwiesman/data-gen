use rand::prelude::Distribution;
use rand::Rng;
use rand_regex::Regex;
use regex_syntax;
use serde::{Deserialize, Serialize, Serializer};
use std::convert::{TryFrom, TryInto};
use std::fmt::{Debug, Formatter};
use thiserror::Error;

/// A valid regular expression that can be sampled for strings
/// which match the pattern.
///
/// # Examples
///
/// ```
/// use data_gen_lib::regex_pattern::RegexPattern;
/// use std::convert::TryInto;
///
/// let pattern: Result<RegexPattern, _> = r"\d{3}".to_string().try_into();
/// assert!(pattern.is_ok());
///
/// let invalid: Result<RegexPattern, _> = r"\d{3".to_string().try_into();
/// assert!(invalid.is_err())
/// ```
#[derive(Clone, Deserialize)]
#[serde(try_from = "IntermediateRegexPattern")]
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
        f.write_str("RegexPattern{")?;
        f.write_str(&self.format)?;
        f.write_str("}")
    }
}

impl<'a> Distribution<String> for RegexPattern {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        rng.sample(&self.regex)
    }
}

impl TryFrom<String> for RegexPattern {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();

        let regex = parser
            .parse(&value)
            .map_err(rand_regex::Error::Syntax)
            .and_then(move |hir| rand_regex::Regex::with_hir(hir, 100));

        match regex {
            Ok(regex) => Ok(RegexPattern {
                regex,
                format: value,
            }),
            Err(err) => Err(Error::Invalid {
                pattern: value,
                source: err,
            }),
        }
    }
}

#[derive(Deserialize)]
struct IntermediateRegexPattern(String);

impl<'a> TryFrom<IntermediateRegexPattern> for RegexPattern {
    type Error = Error;

    fn try_from(value: IntermediateRegexPattern) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<'a> Serialize for RegexPattern {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.format)
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid regular expression: {pattern}")]
    Invalid {
        pattern: String,
        source: rand_regex::Error,
    },
}
