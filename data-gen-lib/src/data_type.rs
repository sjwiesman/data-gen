use crate::interpolator::Interpolator;
use crate::regex_pattern::RegexPattern;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

/// Data Types represent the logical types of
/// fields within a [Schema]. Data types
/// are intended to create realistic looking
/// values for schema fields.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum DataType<'a> {
    /// An homogenous fixed sized collection of a
    /// specified [DataType].
    Array {
        element: Box<DataType<'a>>,
        size: u32,
    },

    /// A simple boolean.
    Boolean,

    Generator {
        format: Interpolator<'a>,
    },

    /// A literal type which always returns
    /// a static value.
    Literal {
        value: &'a str,
    },

    /// A nested structured type comprised of
    /// heterogeneous named fields.
    Object {
        fields: HashMap<&'a str, DataType<'a>>,
    },

    /// An enumeration type that returns
    /// one of a fixed set of values.
    OneOf {
        options: Vec<&'a str>,
    },

    /// A realistic looking phone number of the format
    /// `XXX-XXX-XXXX`.
    PhoneNumber,

    /// A random number from a given range.
    Range {
        from: i32,
        to: i32,
    },

    /// A random value that matches the specified
    /// regular expression.
    ///
    /// # Examples:
    ///
    /// A regular expression that generates strings that
    /// look like URLs.
    ///
    /// ```
    /// # use data_gen_lib::data_type::DataType::Regex;
    /// use data_gen_lib::regex_pattern::RegexPattern;
    /// # use std::convert::{TryFrom, TryInto};
    ///
    /// let _ = Regex {
    ///     pattern: r"www\.[a-z]{3,10}\.(com|org|io)".try_into().unwrap()
    /// };
    /// ```
    Regex {
        pattern: RegexPattern<'a>,
    },

    /// A timestamp formatted string. Unlike other
    /// data types, timestamps always return the
    /// current time. For many applications
    /// it makes sense for timestamps to be
    /// monotonically increasing.
    Timestamp,
}

impl Display for DataType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = serde_json::to_string_pretty(self).map_err(|_| Error)?;
        write!(f, "{}", string)
    }
}
