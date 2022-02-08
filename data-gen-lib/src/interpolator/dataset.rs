use include_dir::{include_dir, Dir, DirEntry, File};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io::ErrorKind;
use std::path::Path;

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/resources");

lazy_static! {
    pub(crate) static ref FULL_DATA_SET: HashMap<String, Vec<&'static str>> = load().unwrap();
}

fn load() -> std::io::Result<HashMap<String, Vec<&'static str>>> {
    let dataset = PROJECT_DIR
        .find("*.json")
        .unwrap()
        .into_iter()
        .filter_map(|entry| match entry {
            DirEntry::Dir(_) => None,
            DirEntry::File(f) => Some(f),
        })
        .map(|file| dataset_name(file.path()).map(|name| (name, file)))
        .collect::<std::io::Result<Vec<_>>>()?
        .into_iter()
        .map(|(category, file)| read(file).map(|data| (category, data)))
        .collect::<std::io::Result<Vec<_>>>()?
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

fn dataset_name(path: &Path) -> std::io::Result<&str> {
    let name = path.to_str().and_then(|name| name.split('.').next());

    match name {
        Some(name) => Ok(name),
        None => Err(std::io::Error::new(
            ErrorKind::Other,
            "invalid path name for dataset",
        )),
    }
}

fn read<'b>(file: &'b File) -> std::io::Result<HashMap<&'b str, Vec<&'b str>>> {
    serde_json::from_slice(file.contents())
        .map_err(|e| std::io::Error::new(ErrorKind::Other, format!("failed to read file {}: {}", file.path().to_str().unwrap(), e)))
}
