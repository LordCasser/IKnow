
use serde_json::{Value};

pub(crate) struct ToutiaoResult {
    // url: String,
    pub(crate) title: String,
}

pub(crate) fn fetch_data() -> Vec<ToutiaoResult>{
    let client = reqwest::blocking::Client::new();

    let response = client
        .get("https://is-lq.snssdk.com/api/suggest_words/?business_id=10016")
        .send().unwrap().text().unwrap();

    // println!("{:#?}", response);

    let data: Value = serde_json::from_str(&*response).unwrap();

    // println!("{:#?}", data);

    let results: Vec<ToutiaoResult> = data["data"][0]["words"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            ToutiaoResult {
                // url: v["id"].as_str().unwrap().to_string(),
                title: v["word"].as_str().unwrap().to_string(),
            }
        })
        .collect();

    results


}