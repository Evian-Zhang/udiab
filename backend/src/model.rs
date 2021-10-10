use super::interfaces::{
    AdvanceSearchOptions, SearchSortBy, SearchedArticleInfo, Snippet, TopArticleInfo, UserError,
    MAX_BODY_LENGTH, MAX_CODE_LENGTH, MAX_KEY_HINTS_COUNT, MAX_TITLE_LENGTH,
    TOP_ARTICLE_INFOS_COUNT,
};
use cang_jie::CANG_JIE;
use chrono::{Duration, Utc};
use search_base::ProjectDocument;
use std::convert::Into;
use tantivy::collector::TopDocs;
use tantivy::{
    query::{BooleanQuery, PhraseQuery, Query, QueryParser, RangeQuery, TermQuery},
    schema::IndexRecordOption,
    DocAddress, IndexReader, Searcher, SnippetGenerator, Term,
};

/// Model for UDIAB project
pub struct UdiabModel {
    /// Index reader, can be retrieved from search_base
    pub reader: IndexReader,
    /// Project document containing `Field` struct for each fields in schema,
    /// can be retrieved from search_base.
    ///
    /// Presented because we would use `get_field` with string comparison otherwise
    pub project_document: ProjectDocument,
}

/// Convert DocAddress to SearchedArticleInfo
fn from_doc_address_to_searched_article_info(
    searcher: &Searcher,
    project_document: &ProjectDocument,
    title_snippet_generator: &SnippetGenerator,
    body_snippet_generator: &SnippetGenerator,
    code_snippet_generator: &SnippetGenerator,
    doc_address: DocAddress,
) -> Result<SearchedArticleInfo, UserError> {
    let doc = searcher
        .doc(doc_address)
        .map_err(|tantivy_error| UserError::UnexpectedTantivy { tantivy_error })?;
    let title_snippet = title_snippet_generator.snippet_from_doc(&doc);
    let title_snippet = Snippet {
        fragments: title_snippet.fragments().to_string(),
        highlighted_positions: title_snippet.highlighted().to_vec(),
    };

    let body_snippet = body_snippet_generator.snippet_from_doc(&doc);
    // If no body snippet is found, returns empty string
    // FIXME: can this branch really exist????
    let body_snippet = Snippet {
        fragments: body_snippet.fragments().to_string(),
        highlighted_positions: body_snippet.highlighted().to_vec(),
    };

    let code_snippet = code_snippet_generator.snippet_from_doc(&doc);
    // If no code snippet if found, returns None
    let code_snippet = if code_snippet.fragments().is_empty() {
        None
    } else {
        Some(Snippet {
            fragments: code_snippet.fragments().to_string(),
            highlighted_positions: code_snippet.highlighted().to_vec(),
        })
    };

    let mut title = None;
    let mut url = None;
    let mut likes = None;
    let mut time = None;
    for field_value in doc.field_values() {
        match field_value.field() {
            field if field == project_document.title => {
                if title_snippet.fragments.is_empty() {
                    title = field_value.value().text()
                }
            }
            field if field == project_document.url => url = field_value.value().text(),
            field if field == project_document.likes => likes = field_value.value().u64_value(),
            field if field == project_document.time => time = field_value.value().date_value(),
            _ => {}
        }
    }
    let title_snippet = if title_snippet.fragments.is_empty() {
        if let Some(title) = title {
            Snippet {
                fragments: title.chars().take(MAX_TITLE_LENGTH).collect::<String>(),
                ..title_snippet
            }
        } else {
            return Err(UserError::Unexpected(format!("Can't find title field")));
        }
    } else {
        title_snippet
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
    let time = if let Some(time) = time {
        *time
    } else {
        return Err(UserError::Unexpected(format!("Can't find time field")));
    };
    Ok(SearchedArticleInfo {
        title_snippet,
        body_snippet,
        code_snippet,
        url,
        likes,
        time,
        address: doc_address.into(),
    })
}

impl UdiabModel {
    /// Get key hints
    ///
    /// For now, we just start a query for title field.
    ///
    /// The returned snippets count will not exceed [`MAX_KEY_HINTS_COUNT`]
    pub fn get_key_hints(&self, key: String) -> Result<Vec<Snippet>, UserError> {
        let searcher = self.reader.searcher();

        let query_parser =
            QueryParser::for_index(&searcher.index(), vec![self.project_document.title]);

        let query = query_parser.parse_query(&key).map_err(|tantivy_error| {
            UserError::UnexpectedTantivy {
                tantivy_error: tantivy_error.into(),
            }
        })?;

        let mut snippet_generator =
            SnippetGenerator::create(&searcher, &query, self.project_document.title)
                .map_err(|tantivy_error| UserError::UnexpectedTantivy { tantivy_error })?;
        snippet_generator.set_max_num_chars(MAX_TITLE_LENGTH);
        let snippets = searcher
            .search(&query, &TopDocs::with_limit(MAX_KEY_HINTS_COUNT))
            .map_err(|tantivy_error| UserError::UnexpectedTantivy { tantivy_error })?
            .into_iter()
            .map(|(_, doc_address)| {
                let doc = searcher
                    .doc(doc_address)
                    .map_err(|tantivy_error| UserError::UnexpectedTantivy { tantivy_error })?;
                let snippet = snippet_generator.snippet_from_doc(&doc);
                Ok(Snippet {
                    fragments: snippet.fragments().to_string(),
                    highlighted_positions: snippet.highlighted().to_vec(),
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(snippets)
    }
    /// Get searched article info
    pub fn get_retrieved_info(
        &self,
        key: String,
        advanced_search_options: AdvanceSearchOptions,
        offset: usize,
        page_size: usize,
    ) -> Result<Vec<SearchedArticleInfo>, UserError> {
        let ProjectDocument {
            title: title_field,
            body: body_field,
            code: code_field,
            likes: likes_field,
            time: time_field,
            ..
        } = self.project_document;
        let searcher = self.reader.searcher();

        let searched_fields = advanced_search_options
            .search_field
            .tantivy_fields(self.project_document);

        let query_parser = QueryParser::for_index(&searcher.index(), searched_fields.clone());

        let query = if advanced_search_options.use_complex_search {
            // When user uses complex search, we just use the built-in parse query
            // to build a query, and NO lenient mode
            query_parser.parse_query(&key).map_err(|tantivy_error| {
                UserError::UnexpectedTantivy {
                    tantivy_error: tantivy_error.into(),
                }
            })?
        } else {
            // When user does not use complex search, we use corresponding
            // tokenizer to tokenize the whole query, and make it a TermQuery
            // or PhraseQuery. For multiple field searching, the final query
            // will be a bool-or query.
            //
            // This query procedure can be found in the source code in tantivy;
            // since issue tantivy-search/tantivy#1162 has not been resolved,
            // we can only extract such logic from its source code

            // For All and Title
            let chinese_tokenizer = self
                .reader
                .searcher()
                .index()
                .tokenizers()
                .get(CANG_JIE)
                .ok_or(UserError::Unexpected(format!(
                    "Unable to find CANG JIE tokenizer"
                )))?;
            // For Code
            let trivial_tokenizer = self
                .reader
                .searcher()
                .index()
                .tokenizers()
                .get("default")
                .ok_or(UserError::Unexpected(format!(
                    "Unable to find default tokenizer"
                )))?;
            let mut subqueries = searched_fields
                .iter()
                .filter_map(|field| {
                    let mut terms = vec![];
                    let tokenizer = match field {
                        field if *field == title_field || *field == body_field => {
                            &chinese_tokenizer
                        }
                        field if *field == code_field => &trivial_tokenizer,
                        // unreachable
                        _ => &trivial_tokenizer,
                    };
                    let mut token_stream = tokenizer.token_stream(&key);
                    token_stream.process(&mut |token| {
                        let term = Term::from_field_text(*field, &token.text);
                        terms.push((token.position, term));
                    });
                    match &terms[..] {
                        [] => None,
                        [(_, term)] => Some(Box::new(TermQuery::new(
                            term.clone(),
                            IndexRecordOption::WithFreqs,
                        )) as Box<dyn Query>),
                        _ => Some(Box::new(PhraseQuery::new_with_offset(terms)) as Box<dyn Query>),
                    }
                })
                .collect::<Vec<Box<dyn Query>>>();
            if let &[_] = &subqueries[..] {
                subqueries.pop().unwrap()
            } else {
                Box::new(BooleanQuery::union(subqueries))
            }
        };

        let mut title_snippet_generator = SnippetGenerator::create(&searcher, &query, title_field)
            .map_err(|tantivy_error| UserError::UnexpectedTantivy { tantivy_error })?;
        title_snippet_generator.set_max_num_chars(MAX_TITLE_LENGTH);
        let mut body_snippet_generator = SnippetGenerator::create(&searcher, &query, body_field)
            .map_err(|tantivy_error| UserError::UnexpectedTantivy { tantivy_error })?;
        body_snippet_generator.set_max_num_chars(MAX_BODY_LENGTH);
        let mut code_snippet_generator = SnippetGenerator::create(&searcher, &query, code_field)
            .map_err(|tantivy_error| UserError::UnexpectedTantivy { tantivy_error })?;
        code_snippet_generator.set_max_num_chars(MAX_CODE_LENGTH);

        let search_collector = TopDocs::with_limit(page_size).and_offset(offset);
        let searched_article_infos = match advanced_search_options.sort_by {
            SearchSortBy::Hot => {
                let search_collector = search_collector.order_by_u64_field(likes_field);
                searcher
                    .search(&query, &search_collector)
                    .map_err(|tantivy_error| UserError::UnexpectedTantivy { tantivy_error })?
                    .into_iter()
                    .map(|(_, doc_address)| {
                        from_doc_address_to_searched_article_info(
                            &searcher,
                            &self.project_document,
                            &title_snippet_generator,
                            &body_snippet_generator,
                            &code_snippet_generator,
                            doc_address,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
            }
            SearchSortBy::Time => {
                // If the field is a `FAST` field but not a `u64` field,
                // search will return successfully but it will return returns
                // a monotonic `u64`-representation (ie. the order is still correct) of the requested field type.
                let search_collector = search_collector.order_by_u64_field(time_field);
                searcher
                    .search(&query, &search_collector)
                    .map_err(|tantivy_error| UserError::UnexpectedTantivy { tantivy_error })?
                    .into_iter()
                    .map(|(_, doc_address)| {
                        from_doc_address_to_searched_article_info(
                            &searcher,
                            &self.project_document,
                            &title_snippet_generator,
                            &body_snippet_generator,
                            &code_snippet_generator,
                            doc_address,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
            }
            SearchSortBy::Relevance => {
                // search_collector is scored by relevance by default
                searcher
                    .search(&query, &search_collector)
                    .map_err(|tantivy_error| UserError::UnexpectedTantivy { tantivy_error })?
                    .into_iter()
                    .map(|(_, doc_address)| {
                        from_doc_address_to_searched_article_info(
                            &searcher,
                            &self.project_document,
                            &title_snippet_generator,
                            &body_snippet_generator,
                            &code_snippet_generator,
                            doc_address,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
            }
        }?;

        Ok(searched_article_infos)
    }

    /// Get the top 10 hot articles from yesterday to today
    pub fn get_top_info(&self) -> Result<Vec<TopArticleInfo>, UserError> {
        let ProjectDocument {
            title: title_field,
            url: url_field,
            likes: likes_field,
            time: time_field,
            ..
        } = self.project_document;

        let searcher = self.reader.searcher();
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
}
