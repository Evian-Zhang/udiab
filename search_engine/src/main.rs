use chrono::{DateTime, NaiveDateTime, Utc};
use search_base::*;
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::io::Read;
use tantivy::doc;

#[derive(Deserialize)]
struct ArticleInfo {
    title: String,
    content: Vec<String>,
    code: Vec<String>,
    url: String,
    date: u64,
    views: u64,
}

#[derive(Deserialize)]
struct IndexerConfig {
    index_store_directory: String,
    data_path: String,
}

fn main() {
    let config_file_path = "./indexer-config.toml";
    let config_str = fs::read_to_string(&config_file_path).expect(&format!(
        "Unable to open config file at {}.",
        &config_file_path
    ));
    let config: IndexerConfig = match toml::from_str(&config_str) {
        Ok(config) => config,
        Err(error) => panic!("Config file parse failed: {}", error),
    };
    let path = &config.index_store_directory;
    // base_result.0 is the index
    // base_result.1 is the struct ProjectDocument handling the fields
    let base_result = index(path).unwrap();

    // extract index writer
    let mut index_writer = base_result.0.writer(50_000_000).unwrap();

    // read documents
    // let mut file = std::fs::File::open("test_files/JianShu.txt").unwrap();
    // let mut buf = vec![];
    // match file.read_to_end(&mut buf) {
    //     Ok(i) => {},
    //     Err(j) => {
    //         panic!("read_to_end error!");
    //     }
    // }
    // let contents = String::from_utf8_lossy(&buf);
    // let contents = file.read_to_string().unwrap();
    let contents = fs::read_to_string(&config.data_path).unwrap();
    let mut lines = contents.lines();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // let mut lines = contents.lines();

    // parse json and write documents into index
    let mut a = 0;
    for line in lines {
        a += 1;
        if line.trim().len() == 0 {
            continue;
        }
        let json_object: ArticleInfo = if let Ok(json_object) = serde_json::from_str(line.trim()) {
            json_object
        } else {
            println!("{}", a);
            return;
        };
        let naive_datetime = NaiveDateTime::from_timestamp(
            (json_object.date / 1000) as i64,
            (json_object.date % 1000) as u32,
        );
        let date = DateTime::from_utc(naive_datetime, Utc);
        index_writer.add_document(doc!(
            base_result.1.title => json_object.title,
            base_result.1.body => json_object.content.concat(),
            base_result.1.code => json_object.code.concat(),
            base_result.1.url => json_object.url,
            base_result.1.time => date,
            base_result.1.likes => json_object.views,
        ));
    }

    // commit index writer
    match index_writer.commit() {
        Ok(i) => {}
        Err(j) => {
            panic!("commit error!");
        }
    }
}

// fn find_next_json_string_idx(s: &str, start_idx: usize) -> usize {
//     let bytes = (&s[start_idx+1..]).as_bytes();
//     let mut title_match = false;
//     let mut title = [false; 9];

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b'{' {
//             title[0] = true;
//             for j in 1..9 {
//                 title[j] = false;
//             }
//             title_match = true;
//         } else if title_match == true {
//             let mut now_index = 0;
//             for j in 1..9 {
//                 if title[j] != true {
//                     now_index = j;
//                     break;
//                 }
//             }
//             if (now_index == 1 && item == b'"') || (now_index == 2 && item == b't') || (now_index == 3 && item == b'i') || (now_index == 4 && item == b't') || (now_index == 5 && item == b'l') || (now_index == 6 && item == b'e') || (now_index == 7 && item == b'"') {
//                 title[now_index] = true;
//             } else if now_index == 8 && item == b':'{
//                 // 返回的位置在'{"title":'的'{'这个位置
//                 return (start_idx + 1) + (i - 8);
//             } else {
//                 title_match = false;
//                 for j in 0..9 {
//                     title[j] = false;
//                 }
//             }
//         }
//     }

//     return (start_idx + 1) + (bytes.len() + 1);
// }
