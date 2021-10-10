use actix_web::{
    dev::HttpResponseBuilder,
    error,
    http::{header, StatusCode},
    HttpResponse,
};
use chrono::{serde::ts_milliseconds::serialize as to_milli_ts, DateTime, Utc};
use derive_more::Display;
use search_base::ProjectDocument;
use serde::{Deserialize, Serialize};
use std::convert::{From, TryFrom};
use std::ops::Range;
use tantivy::{error::TantivyError, schema::Field, DocAddress};

/// Max length of title and/or title snippet (in UTF-8 chars)
pub const MAX_TITLE_LENGTH: usize = 32;

/// Max length of body snippet (in UTF-8 chars)
pub const MAX_BODY_LENGTH: usize = 256;

/// Max length of code snippet (in UTF-8 chars)
pub const MAX_CODE_LENGTH: usize = 256;

/// Max count of key hints
pub const MAX_KEY_HINTS_COUNT: usize = 8;

/// Snippet
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub fragments: String,
    /// Position of highlighted segments in fragments
    ///
    /// This field may be empty if no snippet is matched
    pub highlighted_positions: Vec<Range<usize>>,
}

/// The same structure as [`tantivy::DocAddress`].
///
/// Tantivy's `DocAddress` does not implement `Serialize` trait.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UdiabDocAddress {
    pub segment_ord: u32,
    pub doc_id: u32,
}

impl From<DocAddress> for UdiabDocAddress {
    fn from(doc_address: DocAddress) -> Self {
        let DocAddress {
            segment_ord,
            doc_id,
        } = doc_address;
        Self {
            segment_ord,
            doc_id,
        }
    }
}

impl From<UdiabDocAddress> for DocAddress {
    fn from(udiab_doc_address: UdiabDocAddress) -> Self {
        let UdiabDocAddress {
            segment_ord,
            doc_id,
        } = udiab_doc_address;
        Self {
            segment_ord,
            doc_id,
        }
    }
}

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
    pub title_snippet: Snippet,
    /// Snippet of body
    pub body_snippet: Snippet,
    /// Snippet of code
    ///
    /// If no code is matched, this field is None
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_snippet: Option<Snippet>,
    /// Number of likes
    ///
    /// Used for hot scoring
    pub likes: u64,
    /// Time of this article
    ///
    /// In format of milliseconds in UTC
    #[serde(serialize_with = "to_milli_ts")]
    pub time: DateTime<Utc>,
    /// Doc address
    pub address: UdiabDocAddress,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvanceSearchOptions {
    pub sort_by: SearchSortBy,
    pub search_field: SearchField,
    pub use_complex_search: bool,
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

pub const TOP_ARTICLE_INFOS_COUNT: usize = 10;

/// Article structure used for today's top
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TopArticleInfo {
    /// URL of article
    pub url: String,
    /// Title of article
    pub title: String,
    /// Number of likes
    pub likes: u64,
}

/// Errors which will be sent to user
#[derive(Debug, Display)]
pub enum UserError {
    UnexpectedTantivy {
        tantivy_error: TantivyError,
    },
    #[display(fmt = "Unexpected error: {}", _0)]
    Unexpected(String),
}

impl std::error::Error for UserError {}

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
