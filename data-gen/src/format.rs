use std::fmt::Write;

use anyhow::Result;
use clap::Subcommand;
use data_gen_lib::schema::Schema;
use serde_json::Value;

use self::csv::CsvWriter;
use self::json::JsonWriter;

mod csv;
mod json;

/// Set the output data format.
#[derive(Subcommand)]
pub enum Format {
    /// Write values as CSV to stdout.
    Csv,

    /// Write values as CSV to stdout.
    Json {
        /// Pretty print the JSON records.
        #[clap(short, long)]
        pretty: bool,
    },
}

pub trait Writer {
    fn write(&self, v: Value) -> Result<()>;
}

impl Format {
    pub fn new_writer(&self, _: &Schema) -> Box<dyn Writer> {
        match self {
            Format::Json { pretty } => Box::new(JsonWriter::new(*pretty)),
            Format::Csv => Box::new(CsvWriter),
        }
    }
}

#[allow(dead_code)]
struct InsertInto {
    interpolator: String,
}

#[allow(dead_code)]
impl InsertInto {
    fn new(schema: &Schema) -> Result<InsertInto> {
        let mut insert_into = InsertInto {
            interpolator: String::new(),
        };

        insert_into.interpolator.write_str("INSERT INTO table (")?;
        let columns = schema
            .iter()
            .map(|(field, _)| field)
            .collect::<Vec<_>>()
            .join(",");

        insert_into.interpolator.write_str(&columns)?;
        insert_into.interpolator.write_str(") ")?;
        insert_into.interpolator.write_str("VALUES (")?;

        let place_holders = schema.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        insert_into.interpolator.write_str(&place_holders)?;
        insert_into.interpolator.write_str(")")?;
        Ok(insert_into)
    }
}

impl Writer for InsertInto {
    fn write(&self, _: Value) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use data_gen_lib::data_type::DataType;
    use data_gen_lib::schema::Schema;

    use super::InsertInto;

    #[test]
    fn test_insert_statement() {
        let mut schema = Schema::default();
        schema.with_field("field_1", DataType::Boolean);
        schema.with_field("field_2", DataType::Integer);

        let insert_into = InsertInto::new(&schema);
        assert_eq!(
            remove_whitespace(&insert_into.unwrap().interpolator),
            remove_whitespace("INSERT INTO table (field_1,field_2) VALUES (?,?)")
        )
    }

    fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }
}
