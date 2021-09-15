use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsInfo {
    pub url: String,
    pub title: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvanceSearchOptions {
    pub sort_by: SearchSortBy,
}

#[derive(Deserialize)]
#[serde(try_from = "usize")]
pub enum SearchSortBy {
    Time,
    Hot,
}

impl TryFrom<usize> for SearchSortBy {
    type Error = String;

    fn try_from(discriminant: usize) -> Result<Self, Self::Error> {
        match discriminant {
            0 => Ok(SearchSortBy::Time),
            1 => Ok(SearchSortBy::Hot),
            _ => Err(format!("Unknown discriminant {}.", discriminant)),
        }
    }
}
