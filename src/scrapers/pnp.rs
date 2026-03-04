use crate::transport::client::HttpClient;
use anyhow::Result;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use urlencoding::encode;
use serde::Deserialize;
use crate::models::Product;
use chrono::Utc;

pub struct Pnp {
    pub url: String,
    pub store_code: String,
    pub client: HttpClient,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PnpResponse {
    products: Vec<PnpProduct>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PnpPrice {
    value: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PnpProduct {
    name: String,
    code: String,
    price: PnpPrice,
}

 impl Pnp {
    pub fn new(url: String, client: HttpClient, store_code: String) -> Self {
        Self { url, client, store_code }
    }

    pub async fn search(&self, item: &str) ->Result<()> {
        let query = format!("term={}&maxSuggestions=5&maxProducts=50&storeCode={}&lang=en&curr=ZAR",encode(item), self.store_code);

        let full_url = format!("{}?{}", self.url.trim_end_matches('?'), query);

        let mut headers = HeaderMap::new();
        headers.insert("Referer", HeaderValue::from_static("https://www.pnp.co.za/"));

        let response: PnpResponse = self.client.get_json(&full_url, headers).await?;

        println!("response {:?}", response);

        Ok(())
    }
 }

 impl From<PnpProduct> for Product {
    fn from(p: PnpProduct) -> Self {
        Self {
            name: p.name,
            price: p.price.value,
            retailer: "picknpay".to_string(),
            sku: p.code,
            barcode: None,
            scraped_at: Some(Utc::now())
        }
    }
}

impl From<PnpResponse> for Vec<Product> {
    fn from(p: PnpResponse) -> Self {
        p.products.into_iter().map(Product::from).collect()
    }
}