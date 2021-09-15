use serde::Serialize;

use super::common::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyHintsResponse {
    pub key_hints: Vec<String>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrievedInfoResponse {
    pub news_infos: Vec<NewsInfo>,
}
