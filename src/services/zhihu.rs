use std::fmt::Error;
use serde_json::{Result, Value};


pub(crate) struct SearchResult {
    pub(crate) title: String,
}
#[tokio::main]
pub(crate) async fn fetch_search() -> Vec<SearchResult>{
    let client = reqwest::Client::new();

    let response = client
        .get("https://www.zhihu.com/api/v4/search/top_search")
        .send()
        .await.unwrap().text().await.unwrap();

    let data: Value = serde_json::from_str(&*response).unwrap();


    let results: Vec<SearchResult> = data["top_search"]["words"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            SearchResult {
                // url: v["id"].as_str().unwrap().to_string(),
                title: v["query"].as_str().unwrap().to_string(),
            }
        })
        .collect();

    results

}

pub(crate) struct QuestionResult {
    pub(crate)    title: String,
    pub(crate)   hot: String,
}

#[tokio::main]
pub(crate) async fn fetch_question() -> Vec<QuestionResult>{
    let client = reqwest::Client::new();

    let response = client
        .get("https://www.zhihu.com/api/v3/feed/topstory/hot-lists/total?limit=100")
        .send()
        .await.unwrap().text().await.unwrap();

    // println!("{:#?}", response);

    let data: Value = serde_json::from_str(&*response).unwrap();

    // println!("{:#?}", data);

    let results: Vec<QuestionResult> = data["data"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            QuestionResult {
                hot: v["detail_text"].as_str().unwrap().to_string(),
                title: v["target"]["title"].as_str().unwrap().to_string(),
            }
        })
        .collect();

    results
}