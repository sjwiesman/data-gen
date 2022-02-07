use crate::interpolator::address::search_address;
use crate::interpolator::ancient::search_ancient;

#[derive(Debug)]
pub struct DataSet {
    pub(crate) tag: &'static str,
    pub(crate) options: Vec<&'static str>,
}

impl PartialEq for DataSet {
    fn eq(&self, other: &Self) -> bool {
        self.tag.eq(other.tag)
    }
}

impl Eq for DataSet {}

pub(crate) fn get_dataset(tag: &str) -> Result<&'static DataSet, String> {
    let parts: Vec<&str> = tag.split('.').collect();
    let search = match &parts[..] {
        ["address", ..] => search_address(&parts[1..]),
        ["ancient", ..] => search_ancient(&parts[1..]),
        _ => None,
    };

    if let Some(dataset) = search {
        Ok(dataset)
    } else {
        Err(format!("unknown interpolator tag {}", tag))
    }
}
