use reqwest::{
    header::{self, HeaderValue},
    Error, Response,
};
use serde_json::Map;

extern crate reqwest;

pub async fn get(url: &str, key: String, value: String) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client.get(url).header(key, value).send().await
}

pub async fn post(url: &str, key: String, value: String, body: String) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client.post(url).header(key, value).body(body).send().await
}

pub async fn patch(url: &str, key: String, value: String, body: String) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client.patch(url).header(key, value).body(body).send().await
}

pub async fn put(url: &str, key: String, value: String, body: String) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client.put(url).header(key, value).body(body).send().await
}

pub async fn delete(
    url: &str,
    key: String,
    value: String,
    body: String,
) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    client
        .delete(url)
        .header(key, value)
        .body(body)
        .send()
        .await
}
