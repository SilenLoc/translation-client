use reqwest::{Error, Response};

extern crate reqwest;

pub async fn get(url: &str, key: String, value: String) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client.get(url).header(key, value).send().await
}

pub async fn post(url: &str, key: String, value: String, body: String) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client.post(url).header(key, value).body(body).send().await
}
