use serde::Serialize;

use super::common::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KeyHintsResponse {
    pub key_hints: Vec<Snippet>,
    /// In milli-seconds
    pub duration: u128,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RetrievedInfoResponse {
    pub article_infos: Vec<SearchedArticleInfo>,
    /// In milli-seconds
    pub duration: u128,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopArticleInfoResponse {
    pub top_article_infos: Vec<TopArticleInfo>,
    /// In milli-seconds
    pub duration: u128,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MoreLikeThisResponse {
    pub more_like_this_article_infos: Vec<MoreLikeThisArticleInfo>,
    /// In milli-seconds
    pub duration: u128,
}
