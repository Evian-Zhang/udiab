use std::path::Path;
use std::sync::Arc;

use cang_jie::{CangJieTokenizer, TokenizerOption, CANG_JIE};
use jieba_rs::Jieba;
use tantivy::{
    directory::MmapDirectory,
    schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions, STRING},
    Index,
};

/// Schema for search engine
fn schema() -> Schema {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field(
        "title",
        TextOptions::default().set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer(CANG_JIE)
                .set_index_option(IndexRecordOption::Basic),
        ),
    );

    schema_builder.add_text_field(
        "body",
        TextOptions::default().set_indexing_options(
            TextFieldIndexing::default()
                .set_tokenizer(CANG_JIE)
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        ),
    );

    schema_builder.add_text_field(
        "code",
        TextOptions::default().set_indexing_options(
            TextFieldIndexing::default()
                // untokenized
                .set_tokenizer(STRING.get_indexing_options().unwrap().tokenizer())
                .set_index_option(IndexRecordOption::WithFreqsAndPositions),
        ),
    );

    schema_builder.build()
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

/// Index for search engine
pub fn index<P: AsRef<Path>>(directory_path: P) -> tantivy::Result<Index> {
    let schema = schema();

    let cang_jie_tokenizer = tokenizer();

    let index = Index::open_or_create(MmapDirectory::open(directory_path)?, schema)?;
    index.tokenizers().register(CANG_JIE, cang_jie_tokenizer);

    Ok(index)
}
