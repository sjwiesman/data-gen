mod dataset;

use nom::bytes::complete::{is_not, tag, take_until};
use nom::combinator::recognize;
use rand::distributions::Distribution;
use rand::prelude::*;
use rand::Rng;

use crate::interpolator::dataset::FULL_DATA_SET;

use nom::sequence::delimited;
use nom::IResult;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Interpolator<'a> {
    line: &'a str,
}

impl<'a> TryFrom<&'a str> for Interpolator<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let interpolator = Interpolator { line: value };

        // because tags expand recursively, the only way to
        // verify a string is to run a test expansion.
        let mut rng = rand::rngs::mock::StepRng::new(0, 1);
        match &interpolator.interpolate(&mut rng) {
            Ok(_) => Ok(interpolator),
            Err(e) => Err(e.to_string()),
        }
    }
}

impl<'a> Interpolator<'a> {
    const OPEN_TAG: &'static str = "#{";

    const CLOSE_TAG: &'static str = "}";

    fn interpolate<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<String, String> {
        let mut buffer = String::new();
        let mut stack = Vec::new();

        match Interpolator::until_next_tag(self.line) {
            (prefix, Some(remaining)) => {
                buffer.push_str(prefix);
                stack.push(remaining);
            }

            (prefix, None) => return Ok(prefix.to_string()),
        }

        while let Some(pointer) = stack.pop() {
            if stack.len() == 100 {
                return Err(format!("interpolator {} expands infinitely", self.line));
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

    fn expand<R: Rng + ?Sized>(specifier: &str, rng: &mut R) -> Result<&'static str, String> {
        FULL_DATA_SET
            .get(specifier)
            .ok_or_else(|| format!("no dataset found for specifier {}", specifier))
            .map(|options| options.choose(rng).copied())
            .and_then(|result| {
                result.ok_or_else(|| format!("no options for specifier {}", specifier))
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

impl<'a> Distribution<String> for Interpolator<'a> {
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
