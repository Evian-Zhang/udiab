use super::*;
use crate::interfaces::*;
use chrono::{serde::ts_milliseconds::deserialize as from_milli_ts, DateTime, Utc};
use search_base::ProjectDocument;
use serde::Deserialize;
use std::fs;
use tantivy::doc;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UdiabModelDocument {
    url: String,
    title: String,
    body: String,
    code: String,
    likes: u64,
    #[serde(deserialize_with = "from_milli_ts")]
    time: DateTime<Utc>,
}

fn create_data(manipulator: fn(UdiabModel)) {
    let dir = tempfile::tempdir().unwrap();
    let dir_path = dir.path();

    let (index, project_document) = search_base::index(dir_path).unwrap();
    let ProjectDocument {
        url: url_field,
        title: title_field,
        body: body_field,
        code: code_field,
        likes: likes_field,
        time: time_field,
    } = project_document;
    let mut index_writer = index.writer(100_000_000).unwrap();

    let documents: Vec<UdiabModelDocument> =
        serde_json::from_str(&fs::read_to_string("test_data/main_data.json").unwrap()).unwrap();
    for document in documents {
        let UdiabModelDocument {
            url,
            title,
            body,
            code,
            likes,
            time,
        } = document;
        index_writer.add_document(doc! {
            url_field => url,
            title_field => title,
            body_field => body,
            code_field => code,
            likes_field => likes,
            time_field => time,
        });
    }
    index_writer.commit().unwrap();
    let udiab_model = UdiabModel {
        reader: index.reader().unwrap(),
        project_document,
    };
    manipulator(udiab_model);
}

#[test]
fn test_get_retrieved_info_by_relevance() {
    create_data(|udiab_model| {
        let got = udiab_model.get_retrieved_info(
            "标题".to_string(),
            AdvanceSearchOptions {
                search_field: SearchField::All,
                sort_by: SearchSortBy::Relevance,
                use_complex_search: true,
            },
            0,
            10,
        );
        assert!(got.is_ok());
        let got = got.unwrap();
        let got_url = got.into_iter().map(|info| info.url).collect::<Vec<_>>();
        let expect_url = vec!["url3".to_string(), "url1".to_string(), "url2".to_string()];
        assert_eq!(got_url, expect_url);
    });
}

#[test]
fn test_get_retrieved_info_by_time() {
    create_data(|udiab_model| {
        let got = udiab_model.get_retrieved_info(
            "标题".to_string(),
            AdvanceSearchOptions {
                search_field: SearchField::All,
                sort_by: SearchSortBy::Time,
                use_complex_search: true,
            },
            0,
            10,
        );
        assert!(got.is_ok());
        let got = got.unwrap();
        let got_url = got.into_iter().map(|info| info.url).collect::<Vec<_>>();
        let expect_url = vec!["url1".to_string(), "url2".to_string(), "url3".to_string()];
        assert_eq!(got_url, expect_url);
    });
}

#[test]
fn test_get_retrieved_info_by_hot() {
    create_data(|udiab_model| {
        let got = udiab_model.get_retrieved_info(
            "标题".to_string(),
            AdvanceSearchOptions {
                search_field: SearchField::All,
                sort_by: SearchSortBy::Hot,
                use_complex_search: true,
            },
            0,
            10,
        );
        assert!(got.is_ok());
        let got = got.unwrap();
        let got_url = got.into_iter().map(|info| info.url).collect::<Vec<_>>();
        let expect_url = vec!["url2".to_string(), "url1".to_string(), "url3".to_string()];
        assert_eq!(got_url, expect_url);
    });
}
