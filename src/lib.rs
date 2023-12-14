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
    let mut handles = vec![];

    for word in words {
        handles.push(tokio::spawn(async move {
            let query_string = word.replace(' ', "%20");
            // let voice_string = word.replace(' ', "+");
            // let more_string = word.replace(' ', "_");

            let query_url = format!("{}{}", url_query, query_string);
            // let voice_url = format!("{}{}", url_voice, voice_string);
            // let more_url = format!("{}{}", url_more, more_string);

            let resp = reqwest::get(&query_url).await.unwrap();
            parse_response(resp.text().await.unwrap()).await.unwrap();
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    Ok(())
}

async fn parse_response(resp: String) -> Result<(), Box<dyn Error>> {
    let doc = Document::from(resp.as_str());
    let mut node = doc.find(
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
