use reqwest::{Error, Response};

extern crate reqwest;

pub async fn get(url: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client.get(url).send().await
}

pub async fn post(url: &str, body: String) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client.post(url).body(body).send().await
}
