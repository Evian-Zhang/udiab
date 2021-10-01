use actix_web::{
    dev::HttpResponseBuilder,
    error,
    http::{header, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};
use search_base::ProjectDocument;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use tantivy::{error::TantivyError, schema::Field};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleInfo {
    pub url: String,
    pub title: String,
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
