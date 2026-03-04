use anyhow::Result;
use reqwest::Client;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use serde::de::DeserializeOwned;
use serde_json::Value;

#[derive(Debug,Clone)]
pub struct HttpClient {
    pub inner: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert(
            "Accept",
            HeaderValue::from_static("application/json, text/plain, */*"),
        );
        headers.insert(
            "Accept-Language",
            HeaderValue::from_static("en-US,en;q=0.9"),
        );

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { inner: client }
    }

    pub async fn post_json<T: DeserializeOwned>(
        &self,
        url: &str,
        headers: HeaderMap,
        payload: &Value,
    ) -> Result<T> {
        let response = self
            .inner
            .post(url)
            .headers(headers)
            .json(payload)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, url: &str, headers: HeaderMap) -> Result<T> {
        let response = self
            .inner
            .get(url)
            .headers(headers)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}
