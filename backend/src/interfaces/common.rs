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
use serde_with::rust::display_fromstr::deserialize as deserialize_fromstr;
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
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Snippet {
    pub fragments: String,
    /// Position of highlighted segments in fragments
    ///
    /// This field may be empty if no snippet is matched
    pub highlighted_positions: Vec<Range<usize>>,
}

impl Snippet {
    /// Create a new snippet from fragments and highlighted positions
    ///
    /// The highlighted positions are automatically merged, i.e., 1..2 and 2..3
    /// are merged as 1..3
    pub fn new(fragments: String, highlighted_positions: Vec<Range<usize>>) -> Self {
        let mut merged_highlighted_positions = vec![];
        let mut last_range: Option<Range<usize>> = None;
        for highlighted_position in highlighted_positions {
            if let Some(unwraped_last_range) = last_range {
                if unwraped_last_range.end == highlighted_position.start {
                    last_range = Some(unwraped_last_range.start..highlighted_position.end);
                } else {
                    merged_highlighted_positions.push(unwraped_last_range);
                    last_range = None;
                }
            } else {
                last_range = Some(highlighted_position);
            }
        }
        if let Some(last_range) = last_range {
            merged_highlighted_positions.push(last_range);
        }

        Self {
            fragments,
            highlighted_positions: merged_highlighted_positions,
        }
    }
}

/// The same structure as [`tantivy::DocAddress`].
///
/// Tantivy's `DocAddress` does not implement `Serialize` trait.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UdiabDocAddress {
    // see https://docs.rs/serde_qs/0.8.5/serde_qs/index.html#flatten-workaround
    #[serde(deserialize_with = "deserialize_fromstr")]
    pub segment_ord: u32,
    #[serde(deserialize_with = "deserialize_fromstr")]
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
#[derive(Serialize, Debug)]
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
    pub search_method: SearchMethod,
}

/// The search result is sorted by ...
#[derive(Deserialize)]
#[serde(try_from = "String")]
pub enum SearchSortBy {
    Time,
    Hot,
    Relevance,
}

impl TryFrom<String> for SearchSortBy {
    type Error = String;

    fn try_from(discriminant: String) -> Result<Self, Self::Error> {
        match discriminant.as_str() {
            "0" => Ok(SearchSortBy::Time),
            "1" => Ok(SearchSortBy::Hot),
            "2" => Ok(SearchSortBy::Relevance),
            _ => Err(format!(
                "Unknown discriminant for SearchSortBy: {}.",
                discriminant
            )),
        }
    }
}

/// Search field
#[derive(Deserialize)]
#[serde(try_from = "String")]
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

impl TryFrom<String> for SearchField {
    type Error = String;

    fn try_from(discriminant: String) -> Result<Self, Self::Error> {
        match discriminant.as_str() {
            "0" => Ok(SearchField::Title),
            "1" => Ok(SearchField::Code),
            "2" => Ok(SearchField::All),
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

/// Search Method
#[derive(Deserialize)]
#[serde(try_from = "String")]
pub enum SearchMethod {
    /// Naive search method
    Naive,
    /// Complex search method
    Complex,
    /// Regex search method
    Regex,
}

impl TryFrom<String> for SearchMethod {
    type Error = String;

    fn try_from(discriminant: String) -> Result<Self, Self::Error> {
        match discriminant.as_str() {
            "0" => Ok(SearchMethod::Naive),
            "1" => Ok(SearchMethod::Complex),
            "2" => Ok(SearchMethod::Regex),
            _ => Err(format!(
                "Unknown discriminant for SearchMethod: {}.",
                discriminant
            )),
        }
    }
}

pub const TOP_ARTICLE_INFOS_COUNT: usize = 10;

/// Article structure used for today's top
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TopArticleInfo {
    /// URL of article
    pub url: String,
    /// Title of article
    pub title: String,
    /// Number of likes
    pub likes: u64,
}

/// Article structure used for more like this query
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MoreLikeThisArticleInfo {
    /// URL of article
    pub url: String,
    /// Title of article
    pub title: String,
    /// Body of article
    pub body: String,
    /// Number of likes
    pub likes: u64,
    /// Time of this article
    ///
    /// In format of milliseconds in UTC
    #[serde(serialize_with = "to_milli_ts")]
    pub time: DateTime<Utc>,
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
