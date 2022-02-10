use include_dir::{include_dir, Dir, DirEntry, File};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::Path;
use thiserror::Error;

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources");

lazy_static! {
    pub(crate) static ref FULL_DATA_SET: HashMap<String, Vec<&'static str>> = load().unwrap();
}

fn load() -> Result<HashMap<String, Vec<&'static str>>, Error> {
    let dataset = PROJECT_DIR
        .find("*.json")
        .unwrap()
        .into_iter()
        .filter_map(|entry| match entry {
            DirEntry::Dir(_) => None,
            DirEntry::File(f) => Some(f),
        })
        .map(|file| dataset_name(file.path()).map(|name| (name, file)))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|(category, file)| read(file).map(|data| (category, data)))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flat_map(|(category, data)| {
            data.into_iter().map(move |(section, data)| {
                let tag = format!("#{{{}.{}}}", category, section);
                (tag, data)
            })
        })
        .collect::<HashMap<_, _>>();

    Ok(dataset)
}

fn dataset_name(path: &Path) -> Result<&str, Error> {
    let name = path.to_str().and_then(|name| name.split('.').next());

    match name {
        Some(name) => Ok(name),
        None => Err(Error::InvalidFileName {
            name: path.to_string_lossy().to_string(),
        }),
    }
}

fn read<'b>(file: &'b File) -> Result<HashMap<&'b str, Vec<&'b str>>, Error> {
    serde_json::from_slice(file.contents()).map_err(|e| Error::MalformedDataSet {
        file_name: file.path().to_string_lossy().to_string(),
        source: e,
    })
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid path name for dataset {name}")]
    InvalidFileName { name: String },

    #[error("file {file_name} contains malformed data")]
    MalformedDataSet {
        file_name: String,
        source: serde_json::Error,
    },
}
