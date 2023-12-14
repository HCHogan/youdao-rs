pub mod opt;

use crate::opt::*;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use std::error::Error;

pub async fn run(config: Opt) -> Result<(), Box<dyn Error>> {
    match config.content {
        Content::Word { words } => {
            query_youdao(words).await?;
        }
        Content::Sentence { sentences } => {
            println!("received {:?}", sentences);
        }
    }
    Ok(())
}

async fn query_youdao(words: Vec<String>) -> Result<(), Box<dyn Error>> {
    // let url_more = "http://dict.youdao.com/example/blng/eng/%s";
    // let url_voice = "https://dict.youdao.com/dictvoice?audio=%s&type=2";
    let url_query = "https://dict.youdao.com/w/";

    for word in words {
        let query_string = word.replace(' ', "%20");
        // let voice_string = word.replace(' ', "+");
        // let more_string = word.replace(' ', "_");

        let query_url = format!("{}{}", url_query, query_string);
        // let voice_url = format!("{}{}", url_voice, voice_string);
        // let more_url = format!("{}{}", url_more, more_string);

        let resp = reqwest::get(&query_url).await?;
        parse_response(resp.text().await?).await?;
        // println!("Response for {}: {:?}", word, resp.text().await?);
        // println!("Response for {}: {:?}", word, resp);
    }

    Ok(())
}

async fn parse_response(resp: String) -> Result<(), Box<dyn Error>> {
    let doc = Document::from(resp.as_str());
    let mut node = doc
        .find(
            Attr("id", "phrsListTab")
                .descendant(Class("trans-container"))
                .child(Name("ul")),
        );
        // .nth(0)
        // .expect("No node found with the selector");
    let text = node.next().unwrap().text();
    let text = text.trim().replace("     ", "");
    println!("text:\n{}", text);
    Ok(())
}

// struct QueryParam {
//     word_string: String, // 单词字符串
//     is_chinese: bool,    // 是否是中文
//     is_multi: bool,      // 是否是多个单词
// }
//
// // 定义一个字典结果的结构体
// struct DictResult {
//     word_string: String,       // 单词字符串
//     part_of_speech: String,    // 词性
//     meanings: Vec<String>,     // 含义
//     hints: Option<String>,     // 提示
//     pronounce: Option<String>, // 发音
//     result: String,            // 结果
//     sentences: Vec<String>,    // 例句
// }
//
// // 定义一个从网页解析字典结果的方法
// impl QueryParam {
//     fn parse_web(
//         &self,
//         doc: &select::document::Document,
//         doc_more: Option<&select::document::Document>,
//     ) -> DictResult {
//         // 创建一个空的字典结果
//         let mut ret = DictResult {
//             word_string: self.word_string.clone(),
//             part_of_speech: String::new(),
//             meanings: Vec::new(),
//             hints: None,
//             pronounce: None,
//             result: String::new(),
//             sentences: Vec::new(),
//         };
//
//         if self.is_chinese {
//             // 如果是中文，找到结果
//             for s in doc
//                 .find(select::predicate::Class("trans-container"))
//                 .find(select::predicate::Name("ul"))
//                 .find(select::predicate::Name("p"))
//             {
//                 // 设置词性
//                 ret.part_of_speech = s
//                     .children()
//                     .filter(|c| !c.is(select::predicate::Class("contentTitle")))
//                     .map(|c| c.text())
//                     .collect();
//
//                 // 设置含义
//                 ret.meanings = s
//                     .find(select::predicate::Class("contentTitle"))
//                     .find(select::predicate::Class("search-js"))
//                     .map(|ss| ss.text())
//                     .collect();
//             }
//         } else {
//             // 如果不是中文，检查拼写错误
//             if let Some(hint) = get_hint(doc) {
//                 // 如果有提示，设置提示并返回结果
//                 ret.hints = Some(hint);
//                 return ret;
//             }
//
//             // 找到发音
//             if !self.is_multi {
//                 ret.pronounce = get_pronounce(doc);
//             }
//
//             // 找到结果
//             ret.result = doc
//                 .find(select::predicate::Id("phrsListTab"))
//                 .find(select::predicate::Class("trans-container"))
//                 .find(select::predicate::Name("ul"))
//                 .text();
//         }
//
//         // 显示例句
//         if let Some(doc_more) = doc_more {
//             // 如果有更多的文档，从中获取例句
//             ret.sentences = self.get_sentences(doc_more);
//         } else {
//             // 否则，从当前文档获取例句
//             ret.sentences = self.get_sentences(doc);
//         }
//
//         // 返回结果
//         ret
//     }
// }
//
// // 定义一个从网页获取拼写错误提示的函数
// fn get_hint(doc: &select::document::Document) -> Option<String> {
//     // TODO: 实现这个函数
//     None
// }
//
// // 定义一个从网页获取发音的函数
// fn get_pronounce(doc: &select::document::Document) -> Option<String> {
//     // TODO: 实现这个函数
//     None
// }
//
// // 定义一个从网页获取例句的方法
// impl QueryParam {
//     fn get_sentences(&self, doc: &select::document::Document) -> Vec<String> {
//         // TODO: 实现这个方法
//         Vec::new()
//     }
// }
