use std::io::{stdin, Read, Write};
use std::ops::Div;

use anyhow::{Context, Result};
use rand::{thread_rng, Rng};

use clap::Parser;
use data_gen_lib::generator::DataGenerator;
use data_gen_lib::schema::Schema;

/// Generates realistic looking fake JSON data.
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// A file with the schema for data generation. If no
    /// schema file is provide, it will be read from stdin.
    #[clap(default_value = "-")]
    schema: String,

    /// If set, a JSON value will be produced at a
    /// given rate per second.
    #[clap(short, long)]
    rate: Option<u32>,

    /// Pretty print output JSON.
    #[clap(short, long)]
    pretty: bool,
}

fn main() -> Result<()> {
    let args: Args = Args::parse();

    let data = &read_schema(&args.schema)?;

    let schema: Schema = serde_json::from_str(data)
        .with_context(move || format!("failed to deserialize JSON schema\n{}", data.clone()))?;

    let gen = DataGenerator::json(&schema);

    let print = |v| {
        if args.pretty {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            serde_json::to_writer_pretty(std::io::stdout(), &v)?;
            writeln!(&mut stdout)
        } else {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            serde_json::to_writer(std::io::stdout(), &v)?;
            writeln!(&mut stdout)
        }
    };

    if let Some(rate) = args.rate {
        let sleep_duration = std::time::Duration::from_secs(1).div(rate);
        loop {
            print(thread_rng().sample(&gen))?;
            std::thread::sleep(sleep_duration)
        }
    } else {
        print(thread_rng().sample(&gen))?;
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
