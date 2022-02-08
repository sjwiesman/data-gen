pub mod data_type;
pub mod generator;
pub mod interpolator;
mod physical;
pub mod regex_pattern;
pub mod schema;

#[cfg(test)]
mod tests {
    use crate::data_type::DataType::{
        Array, Boolean, Generator, Literal, Object, OneOf, Range, Regex,
    };
    use crate::generator::DataGenerator;
    use crate::schema::Schema;
    use rand::prelude::*;
    use rand::rngs::mock::StepRng;
    use serde_json::{json, Value};

    #[test]
    fn it_generates() {
        let mut schema = Schema::default();
        schema
            .with_field(
                "array",
                Array {
                    element: Box::new(Literal { value: "a" }),
                    size: 2,
                },
            )
            .with_field("boolean", Boolean)
            .with_field(
                "generator",
                Generator {
                    format: "#{zelda.games} is my favorite zelda game!"
                        .try_into()
                        .unwrap(),
                },
            )
            .with_field("literal", Literal { value: "hello" })
            .with_field("range", Range { from: 1, to: 10 })
            .with_field(
                "regex",
                Regex {
                    pattern: r"\d{3}".try_into().unwrap(),
                },
            )
            .with_field(
                "object",
                Object {
                    fields: vec![("field", Literal { value: "b" })]
                        .into_iter()
                        .collect(),
                },
            )
            .with_field(
                "one_of",
                OneOf {
                    options: vec!["coffee", "tea", "milk"],
                },
            );

        let gen: DataGenerator<Value> = unsafe { DataGenerator::new(&schema) };

        let mut rng = StepRng::new(2, 1);
        let value = rng.sample(gen);

        let expected = json!({
            "array": ["a", "a"],
            "boolean": false,
            "generator": "A Link to the Past is my favorite zelda game!",
            "literal": "hello",
            "range": 1,
            "regex": "000",
            "object": {
                "field": "b"
            },
            "one_of": "coffee"
        });

        assert_eq!(expected, value)
    }
}
