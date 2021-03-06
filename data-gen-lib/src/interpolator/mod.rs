mod dataset;

use nom::bytes::complete::{is_not, tag, take_until};
use nom::combinator::recognize;
use rand::distributions::Distribution;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::interpolator::dataset::FULL_DATA_SET;

use nom::sequence::delimited;
use nom::IResult;

use serde::{Deserialize, Serialize};

use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Interpolator(String);

impl<'a> TryFrom<&'a str> for Interpolator {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let interpolator = Interpolator(value.to_owned());

        // because tags expand recursively, the only way to
        // verify a string is to run a test expansion.
        let mut rng = rand::rngs::mock::StepRng::new(0, 1);
        match &interpolator.interpolate(&mut rng) {
            Ok(_) => Ok(interpolator),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("interpolation format {format} expanded infinitely")]
    InfiniteExpansion { format: String },

    #[error("unknown interpolation format specifier {tag}")]
    UnknownFormatSpecifier { tag: String },

    #[error("interpolation format specifier {tag} has no valid options")]
    EmptyDataSet { tag: String },
}

impl Interpolator {
    const OPEN_TAG: &'static str = "#{";

    const CLOSE_TAG: &'static str = "}";

    fn interpolate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<String, Error> {
        let mut buffer = String::new();
        let mut stack = Vec::new();

        match Interpolator::until_next_tag(&self.0) {
            (prefix, Some(remaining)) => {
                buffer.push_str(prefix);
                stack.push(remaining);
            }

            (prefix, None) => return Ok(prefix.to_string()),
        }

        while let Some(pointer) = stack.pop() {
            if stack.len() == 100 {
                return Err(Error::InfiniteExpansion {
                    format: self.0.to_string(),
                });
            }

            match Interpolator::until_next_tag(pointer) {
                (prefix, Some(remaining)) => {
                    buffer.push_str(prefix);
                    match Interpolator::next_tag(remaining) {
                        (Some(specifier), following) => {
                            let expanded = Interpolator::expand(specifier, rng)?;
                            stack.push(following);
                            stack.push(expanded);
                        }
                        (None, following) => {
                            stack.push(following);
                        }
                    }
                }

                (prefix, None) => {
                    buffer.push_str(prefix);
                }
            }
        }

        Ok(buffer)
    }

    fn expand<R: Rng + ?Sized>(specifier: &str, rng: &mut R) -> Result<&'static str, Error> {
        FULL_DATA_SET
            .get(specifier)
            .ok_or_else(move || Error::UnknownFormatSpecifier {
                tag: specifier.to_string(),
            })
            .map(|options| options.choose(rng).copied())
            .and_then(|result| {
                result.ok_or_else(|| Error::EmptyDataSet {
                    tag: specifier.to_string(),
                })
            })
    }

    fn until_next_tag(input: &str) -> (&str, Option<&str>) {
        let result: IResult<&str, &str> = take_until("#{")(input);
        match result {
            Ok((remaining, prefix)) => (prefix, Some(remaining)),
            Err(_) => (input, None),
        }
    }

    fn next_tag(input: &str) -> (Option<&str>, &str) {
        let result: IResult<&str, &str> = recognize(delimited(
            tag(Interpolator::OPEN_TAG),
            is_not(Interpolator::CLOSE_TAG),
            tag(Interpolator::CLOSE_TAG),
        ))(input);

        match result {
            Ok((following, tag)) => (Some(tag), following),
            Err(_) => (None, input),
        }
    }
}

impl Distribution<String> for Interpolator {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> String {
        let result = self.interpolate(rng);
        result.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::interpolator::Interpolator;

    #[test]
    fn test_interpolator() {
        let interpolator: Result<Interpolator, String> = "#{zelda.games}".try_into();
        assert!(interpolator.is_ok())
    }
}
