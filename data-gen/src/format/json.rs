use serde_json::Value;
use std::io::Write;

use super::Writer;
use anyhow::{Context, Result};

pub struct JsonWriter {
    pretty: bool,
}

impl JsonWriter {
    pub fn new(pretty: bool) -> JsonWriter {
        JsonWriter { pretty }
    }
}

impl Writer for JsonWriter {
    fn write(&self, v: Value) -> Result<()> {
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        if self.pretty {
            serde_json::to_writer_pretty(&mut stdout, &v)
                .context("failed to serialize value as JSON")?;
        } else {
            serde_json::to_writer(&mut stdout, &v).context("failed to serialize value as JSON")?;
        }

        writeln!(&mut stdout).context("failed to write newline")
    }
}
