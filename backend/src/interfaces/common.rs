use actix_web::{
    dev::HttpResponseBuilder,
    error,
    http::{header, StatusCode},
    HttpResponse,
};
use chrono::{serde::ts_milliseconds::serialize as to_milli_ts, DateTime, Utc};
use derive_more::{Display, Error};
use search_base::ProjectDocument;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ops::Range;
use tantivy::{error::TantivyError, schema::Field};

/// Article structure used for searching
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchedArticleInfo {
    /// URL of article
    pub url: String,
    /// Snippet of title
    ///
    /// If only other components are matched, and title has no snippet,
    /// this field is the whole title.
    // TODO: May be limit the length if is whole title?
    pub title_snippet: String,
    /// Highlighted positions of title (end is excluded)
    ///
    /// If only other components are matched, and title has no snippet,
    /// this field is empty list
    pub title_highlighted_positions: Vec<Range<usize>>,
    /// Snippet of body
    pub body_snippet: String,
    /// Highlighted poisitions of body
    pub body_highlighted_positions: Vec<Range<usize>>,
    /// Snippet of code
    /// 
    /// If no code is matched, this field is empty string
    pub code_snippet: String,
    /// Highlighted poisitions of code
    pub code_highlighted_positions: Vec<Range<usize>>,
    /// Number of likes
    ///
    /// Used for hot scoring
    pub likes: usize,
    /// Time of this article
    ///
    /// In format of milliseconds in UTC
    #[serde(serialize_with = "to_milli_ts")]
    pub time: DateTime<Utc>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvanceSearchOptions {
    pub sort_by: SearchSortBy,
    pub search_field: SearchField,
}

/// The search result is sorted by ...
#[derive(Deserialize)]
#[serde(try_from = "usize")]
pub enum SearchSortBy {
    Time,
    Hot,
    Relevance,
}

impl TryFrom<usize> for SearchSortBy {
    type Error = String;

    fn try_from(discriminant: usize) -> Result<Self, Self::Error> {
        match discriminant {
            0 => Ok(SearchSortBy::Time),
            1 => Ok(SearchSortBy::Hot),
            2 => Ok(SearchSortBy::Relevance),
            _ => Err(format!(
                "Unknown discriminant for SearchSortBy: {}.",
                discriminant
            )),
        }
    }
}

/// Search field
#[derive(Deserialize)]
#[serde(try_from = "usize")]
pub enum SearchField {
    /// Only search title
    Title,
    /// Only search code
    Code,
    /// Search all fields.
    ///
    /// Including title, body, code.
    All,
}

impl TryFrom<usize> for SearchField {
    type Error = String;

    fn try_from(discriminant: usize) -> Result<Self, Self::Error> {
        match discriminant {
            0 => Ok(SearchField::Title),
            1 => Ok(SearchField::Code),
            2 => Ok(SearchField::All),
            _ => Err(format!(
                "Unknown discriminant for SearchField: {}.",
                discriminant
            )),
        }
    }
}

impl SearchField {
    /// Get corresponding tantivy::schema::Field
    pub fn tantivy_fields(&self, project_document: ProjectDocument) -> Vec<Field> {
        let ProjectDocument {
            title, body, code, ..
        } = project_document;

        match &self {
            SearchField::Title => vec![title],
            SearchField::Code => vec![code],
            SearchField::All => vec![title, body, code],
        }
    }
}

/// Errors which will be sent to user
#[derive(Debug, Display, Error)]
pub enum UserError {
    UnexpectedTantivy { tantivy_error: TantivyError },
}

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
