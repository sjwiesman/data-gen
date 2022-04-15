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
