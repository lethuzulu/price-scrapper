use crate::{models::Product, transport::client::HttpClient};
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use serde_json::json;

pub struct Checkers {
    pub url: String,
    client: HttpClient,
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

        let response: CheckersResponse = self.client.get(&self.url, headers, &json_payload).await?;

        Ok(response.into())
    }
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
    pub display_name: String,
    pub description: Option<String>,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub price_factor: u32,
    pub price_without_decimal: u32,
    pub currency: String,
    pub currency_symbol: String,
    pub discount: u32,
    pub old_price: u32,
}

impl From<CheckersResponse> for Vec<Product> {
    fn from(value: CheckersResponse) -> Self {
        value.products
            .into_iter()
            .map(|p| Product { name: p.name })
            .collect()
    }
}
