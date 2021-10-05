use super::interfaces::{TopArticleInfo, UserError, TOP_ARTICLE_INFOS_COUNT};
use chrono::{Duration, Utc};
use search_base::ProjectDocument;
use tantivy::collector::TopDocs;
use tantivy::{query::RangeQuery, IndexReader};

/// Get the top 10 hot articles from yesterday to today
pub fn get_top_info(
    reader: &IndexReader,
    project_document: &ProjectDocument,
) -> Result<Vec<TopArticleInfo>, UserError> {
    let ProjectDocument {
        title: title_field,
        url: url_field,
        likes: likes_field,
        time: time_field,
        ..
    } = *project_document;

    let searcher = reader.searcher();
    let current_time = Utc::now();
    let yesterday = current_time - Duration::days(1);
    let docs_in_today =
        RangeQuery::new_i64(time_field, yesterday.timestamp()..current_time.timestamp());
    let today_top_docs = searcher
        .search(
            &docs_in_today,
            &TopDocs::with_limit(TOP_ARTICLE_INFOS_COUNT).order_by_u64_field(likes_field),
        )
        .map_err(|tantivy_error| UserError::UnexpectedTantivy { tantivy_error })?;
    let top_article_infos = today_top_docs
        .into_iter()
        .map(|(_, top_doc_address)| {
            let top_doc = searcher
                .doc(top_doc_address)
                .map_err(|tantivy_error| UserError::UnexpectedTantivy { tantivy_error })?;
            let mut title = None;
            let mut url = None;
            let mut likes = None;
            for field_value in top_doc.field_values() {
                match field_value.field() {
                    field if field == title_field => title = field_value.value().text(),
                    field if field == url_field => url = field_value.value().text(),
                    field if field == likes_field => likes = field_value.value().u64_value(),
                    _ => {}
                }
            }
            let title = if let Some(title) = title {
                title.to_string()
            } else {
                return Err(UserError::Unexpected(format!("Can't find title field")));
            };
            let url = if let Some(url) = url {
                url.to_string()
            } else {
                return Err(UserError::Unexpected(format!("Can't find url field")));
            };
            let likes = if let Some(likes) = likes {
                likes
            } else {
                return Err(UserError::Unexpected(format!("Can't find likes field")));
            };
            Ok(TopArticleInfo { title, url, likes })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(top_article_infos)
}
