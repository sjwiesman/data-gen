use rand::prelude::Distribution;
use rand::Rng;
use rand_regex::Regex;
use regex_syntax;
use serde::{Deserialize, Serialize, Serializer};
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
/// let pattern: Result<RegexPattern, _> = r"\d{3}".try_into();
/// assert!(pattern.is_ok());
///
/// let invalid: Result<RegexPattern, _> = r"\d{3".try_into();
/// assert!(invalid.is_err())
/// ```
#[derive(Clone, Deserialize)]
#[serde(try_from = "IntermediateRegexPattern")]
pub struct RegexPattern<'a> {
    regex: Regex,
    format: &'a str,
}

impl PartialEq for RegexPattern<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.format.eq(other.format)
    }
}

impl Eq for RegexPattern<'_> {}

impl Debug for RegexPattern<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("RegexPattern{")?;
        f.write_str(self.format)?;
        f.write_str("}")
    }
}

impl<'a> Distribution<String> for RegexPattern<'a> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        rng.sample(&self.regex)
    }
}

impl<'a> TryFrom<&'a str> for RegexPattern<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut parser = regex_syntax::ParserBuilder::new().unicode(false).build();

        let regex = parser
            .parse(value)
            .map_err(rand_regex::Error::Syntax)
            .and_then(move |hir| rand_regex::Regex::with_hir(hir, 100))
            .map_err(move |err| format!("invalid regular expression: {}", err))?;

        Ok(RegexPattern {
            regex,
            format: value,
        })
    }
}

#[derive(Deserialize)]
struct IntermediateRegexPattern<'a>(&'a str);

impl<'a> TryFrom<IntermediateRegexPattern<'a>> for RegexPattern<'a> {
    type Error = String;

    fn try_from(value: IntermediateRegexPattern<'a>) -> Result<Self, Self::Error> {
        value.0.try_into()
    }
}

impl<'a> Serialize for RegexPattern<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.format)
    }
}
