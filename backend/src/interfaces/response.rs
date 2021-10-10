use serde::Serialize;

use super::common::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyHintsResponse {
    pub key_hints: Vec<Snippet>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrievedInfoResponse {
    pub article_infos: Vec<SearchedArticleInfo>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopArticleInfoResponse {
    pub top_article_infos: Vec<TopArticleInfo>,
}
