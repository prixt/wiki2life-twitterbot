use std::error::Error;

pub fn read_wiki() -> Result<(String, String, String), Box<dyn Error>> {
    let body: serde_json::Value = 
        reqwest::get("https://en.wikipedia.org/api/rest_v1/page/random/summary")?
        .json()?;

    let title = body["title"].as_str().unwrap();
    let extract = body["extract"].as_str().unwrap();
    let url = body["content_urls"]["desktop"]["page"]
        .as_str().unwrap();

    Ok((title.to_string(), extract.to_string(), url.to_string()))
}