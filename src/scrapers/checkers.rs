use crate::{models::Product, transport::client::HttpClient};
use anyhow::Result;
use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::json;

pub struct Checkers {
    pub url: String,
    client: HttpClient,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckersResponse {
    pub products: Vec<CheckersProduct>,
    pub total_count: u32,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CheckersProduct {
    pub id: String,
    pub store_id: String,
    pub name: String,
    pub price_factor: u32,
    pub price_without_decimal: u32,
    pub article_number: Option<String>,
    pub barcodes: Option<Vec<String>>,
}

impl Checkers {
    pub fn new(url: String, client: HttpClient) -> Self {
        Self { url, client }
    }

    pub async fn search(&self, item: &str) -> Result<Vec<Product>> {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Referer",
            HeaderValue::from_static("https://www.checkers.co.za/"),
        );
        headers.insert(
            "Origin",
            HeaderValue::from_static("https://www.checkers.co.za"),
        );

        let json_payload = json!({
            // "storeContexts": [],
            "filterData": {
              "filter": {
                "showAllDisplayVariants": false,
                "showNotRangedProducts": false,
                "productListSource": {
                  "search": item
                },
                "paginationOptions": {
                  "page": 0,
                  "pageSize": 2
                },
                "filterOptions": {
                  "dealsOnly": false,
                  "serviceOptions": [],
                  "facetOptions": []
                },
                "sortOptions": null
              },
              "displayOptions": {}
            },
            "forYouBonusBuyIds": []
        });

        let response: CheckersResponse = self
            .client
            .post_json(&self.url, headers, &json_payload)
            .await?;

        Ok(response.into())
    }
}



impl From<CheckersProduct> for Product {
    fn from(p: CheckersProduct) -> Self {
        let price = p.price_without_decimal as f64 / p.price_factor as f64;
        Product {
            canonical_name: None,
            name: p.name,
            price,
            retailer: "checkers".to_string(),
            sku: p.article_number.unwrap_or_default(),
            barcode: p.barcodes.and_then(|b| b.into_iter().next()),
            scraped_at: Some(Utc::now()),
        }
    }
}

impl From<CheckersResponse> for Vec<Product> {
    fn from(value: CheckersResponse) -> Self {
        value.products.into_iter().map(Product::from).collect()
    }
}
