use crate::interpolator::dataset::DataSet;
use lazy_static::lazy_static;

lazy_static! {
    static ref CITY_PREFIX: DataSet = DataSet {
        tag: "#{address.city_prefix}",
        options: vec!["North", "East", "West", "South", "New", "Lake", "Port"],
    };
    static ref CITY_SUFFIX: DataSet = DataSet {
        tag: "city_suffix",
        options: vec![
            "town", "ton", "land", "ville", "berg", "burgh", "borough", "bury", "view", "port",
            "mouth", "stad", "furt", "chester", "mouth", "fort", "haven", "side", "shire"
        ]
    };
}

pub(crate) fn search_address(tag: &[&str]) -> Option<&'static DataSet> {
    match tag {
        ["city_prefix"] => Some(&CITY_PREFIX),
        ["city_suffix"] => Some(&CITY_SUFFIX),
        _ => None,
    }
}
