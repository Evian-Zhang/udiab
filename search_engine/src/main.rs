use search_base::*;
use std::io::Read;
use serde_json::{Value};
use tantivy::{
    doc
};

fn main() {
    
    let path = "index_dir";
    // base_result.0 is the index
    // base_result.1 is the struct ProjectDocument handling the fields
    let base_result = index(path).unwrap();

    // extract index writer
    let mut index_writer = base_result.0.writer(50_000_000).unwrap();

    // read documents
    let mut file = std::fs::File::open("test_files\\JianShu.txt").unwrap();
    let mut buf = vec![];
    match file.read_to_end(&mut buf) {
        Ok(i) => {},
        Err(j) => {
            panic!("read_to_end error!");
        }
    }
    let contents = String::from_utf8_lossy(&buf);
    let mut lines = contents.lines();
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // let mut lines = contents.lines();
    
    // parse json and write documents into index
    loop {
        let line = match lines.next() {
            Some(i) => i,
            None => {
                break;
            }
        };
        let json_object: Value = serde_json::from_str(line).unwrap();
        index_writer.add_document(doc!(
            base_result.1.title => match json_object["title"].as_str() {
                Some(s) => s,
                _ => ""
            },
            base_result.1.body => match json_object["content"].as_str() {
                Some(s) => s,
                _ => ""
            },
            base_result.1.code => match json_object["code"].as_str() {
                Some(s) => s,
                _ => ""
            },
            base_result.1.url => match json_object["url"].as_str() {
                Some(s) => s,
                _ => ""
            },
            base_result.1.time => match json_object["date"].as_str() {
                Some(s) => s,
                _ => ""
            },
            base_result.1.likes => match json_object["views"].as_u64() {
                Some(s) => s,
                _ => 0
            }
        ));
    }

    // commit index writer
    match index_writer.commit() {
        Ok(i) => {},
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