use serde_json::Value;

use super::Writer;
use anyhow::{bail, Context, Result};

pub struct CsvWriter;

impl Writer for CsvWriter {
    fn write(&self, v: Value) -> Result<()> {
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        let mut writer = csv::Writer::from_writer(&mut stdout);

        for (name, field) in v.as_object().unwrap().into_iter() {
            match field {
                Value::Null => writer.write_field("")?,
                Value::Bool(b) => writer.write_field(b.to_string())?,
                Value::Number(n) => {
                    if n.is_f64() {
                        writer.write_field(n.as_f64().unwrap().to_string())?
                    } else if n.is_i64() {
                        writer.write_field(n.as_i64().unwrap().to_string())?
                    } else if n.is_u64() {
                        writer.write_field(n.as_u64().unwrap().to_string())?
                    } else {
                        unreachable!("A number should always be f64, i64, or u64")
                    }
                }
                Value::String(string) => writer.write_field(string)?,
                Value::Array(_) => bail!(
                    "Cannot seriailize field {}, csv output does not support array types",
                    name
                ),
                Value::Object(_) => bail!(
                    "Cannot seriailize field {}, csv output does not support map types",
                    name
                ),
            }
        }

        writer
            .write_record(None::<&[u8]>)
            .context("failed to write CSV")
    }
}
