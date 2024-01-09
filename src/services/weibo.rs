use reqwest::header::HeaderValue;

pub(crate) struct WeiboResult {
    pub(crate) url: String,
    pub(crate) title: String,
}
pub(crate) fn fetch_data() -> Vec<WeiboResult>{
    let client = reqwest::blocking::Client::new();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::COOKIE, HeaderValue::from_static("SUB=_2AkMWJrkXf8NxqwJRmP8SxWjnaY12zwnEieKgekjMJRMxHRl-yj9jqmtbtRB6PaaX-IGp-AjmO6k5cS-OH2X9CayaTzVD"));

    let response = client
        .get("https://s.weibo.com/top/summary")
        .headers(headers)
        .send().unwrap().text().unwrap();

    // println!("{:#?}", response);

    let re = regex::Regex::new(r#"<a href="(/weibo\?q=[^"]+)".*?>(.+)</a>"#).unwrap();
    let weibo_result: Vec<WeiboResult> = re.captures_iter(&response)
        .map(|cap| {
            WeiboResult {
                url: "https://s.weibo.com".to_string() + &*cap.get(1).map_or("", |m| m.as_str()).to_string(),
                title: cap.get(2).map_or("", |m| m.as_str()).to_string(),
            }
        })
        .collect();

    // for word in words  {
    //     println!("url:{} | title{}",word.url,word.title);
    // }


    // Ok(())
    weibo_result
}