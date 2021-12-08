use std::path::Path;
use std::sync::Arc;

use cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE};
use jieba_rs::Jieba;
use tantivy::{
    directory::MmapDirectory,
    schema::{
        Field, IndexRecordOption, Schema, TextFieldIndexing, TextOptions, FAST, INDEXED, STORED,
        STRING,
    },
    tokenizer::SimpleTokenizer,
    Index,
};

#[derive(Clone, Copy)]
pub struct ProjectDocument {
    pub title: Field,
    pub body: Field,
    pub code: Field,
    pub url: Field,
    pub time: Field,
    pub likes: Field,
}

/// Schema for search engine
fn schema() -> (Schema, ProjectDocument) {
    let mut schema_builder = Schema::builder();

    let title = schema_builder.add_text_field(
        "title",
        TextOptions::default().set_stored().set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer(CANG_JIE)
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        ),
    );

    let body = schema_builder.add_text_field(
        "body",
        TextOptions::default().set_stored().set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer(CANG_JIE)
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        ),
    );

    let code = schema_builder.add_text_field(
        "code",
        TextOptions::default().set_stored().set_indexing_options(
            TextFieldIndexing::default()
                // untokenized
                .set_tokenizer("naivetokenizer")
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        ),
    );

    let url = schema_builder.add_text_field(
        "url",
        TextOptions::default().set_stored(), // default impl does not gen index
    );

    let time = schema_builder.add_date_field("time", INDEXED | STORED | FAST);

    let likes = schema_builder.add_u64_field("likes", FAST | STORED);

    let project_document = ProjectDocument {
        title,
        body,
        code,
        url,
        time,
        likes,
    };

    (schema_builder.build(), project_document)
}

/// Tokenizer for Chinese sentences
fn tokenizer() -> CangJieTokenizer {
    // Modify this to configure Chinese dict
    let jieba_dict = Jieba::new();

    CangJieTokenizer {
        worker: Arc::new(jieba_dict),
        option: TokenizerOption::Unicode,
    }
}

/// Index for search engine.
///
/// Returns the `tantivy::Index` stucture for reading/writing
/// and `ProjectDocument` for `Field` accessing.
pub fn index<P: AsRef<Path>>(directory_path: P) -> tantivy::Result<(Index, ProjectDocument)> {
    let (schema, project_document) = schema();

    let cang_jie_tokenizer = tokenizer();

    let index = Index::open_or_create(MmapDirectory::open(directory_path)?, schema)?;
    index.tokenizers().register(CANG_JIE, cang_jie_tokenizer);
    index
        .tokenizers()
        .register("naivetokenizer", SimpleTokenizer);

    Ok((index, project_document))
}
