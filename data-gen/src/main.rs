mod format;

use std::io::{stdin, Read};
use std::ops::Div;

use anyhow::{Context, Result};
use rand::{thread_rng, Rng};

use clap::Parser;
use data_gen_lib::generator::DataGenerator;
use data_gen_lib::schema::Schema;

use crate::format::Format;

/// Generates realistic looking fake JSON data.
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Args {
    /// The output data format.
    #[clap(subcommand)]
    format: Format,

    /// A file with the schema for data generation. If no
    /// schema file is provide, it will be read from stdin.
    #[clap(short, long, default_value = "-")]
    schema: String,

    /// If set, a JSON value will be produced at a
    /// given rate per second.
    #[clap(short, long)]
    rate: Option<u32>,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let data = &read_schema(&args.schema)?;

    let schema: Schema = serde_json::from_str(data)
        .with_context(move || format!("failed to deserialize schema\n{}", data.clone()))?;

    let gen = DataGenerator::new(&schema);

    let writer = args.format.new_writer(&schema);

    if let Some(rate) = args.rate {
        let sleep_duration = std::time::Duration::from_secs(1).div(rate);
        loop {
            writer.write(thread_rng().sample(&gen))?;
            std::thread::sleep(sleep_duration)
        }
    } else {
        writer.write(thread_rng().sample(&gen))?;
        Ok(())
    }
}

fn read_schema(schema_path: &str) -> Result<String> {
    if schema_path != "-" {
        std::fs::read_to_string(schema_path)
            .with_context(|| format!("failed to read file: {}", schema_path))
    } else {
        let mut buf: String = String::new();
        let _ = stdin()
            .lock()
            .read_to_string(&mut buf)
            .with_context(|| "failed to read from stdin")?;
        Ok(buf)
    }
}
