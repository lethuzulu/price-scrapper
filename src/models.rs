use chrono::{DateTime, Utc};

pub struct Product {
    pub canonical_name: Option<String>,
    pub name: String,
    pub price: f64,
    pub retailer: String,
    pub sku: String,
    pub barcode: Option<String>,
    pub scraped_at: Option<DateTime<Utc>>,
}
