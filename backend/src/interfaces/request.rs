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
    pub advanced_search_options: AdvanceSearchOptions,
    pub offset: usize,
    pub page_size: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MoreLikeThisRequest {
    #[serde(flatten)]
    pub address: UdiabDocAddress,
    pub offset: usize,
    pub page_size: usize,
}
