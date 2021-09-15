use serde::Deserialize;

use super::common::*;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyHintsRequest {
    pub key: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrievedInfoRequest {
    pub key: String,
    #[serde(flatten)]
    pub advanced_serach_options: AdvanceSearchOptions,
    pub offset: usize,
    pub page_size: usize,
}
