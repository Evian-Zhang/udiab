use actix_web::{
    dev::HttpResponseBuilder,
    error,
    http::{header, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
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

/// Errors which will be sent to user
#[derive(Debug, Display, Error)]
pub enum UserError {}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
