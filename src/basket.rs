use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct BasketItem {
    pub canonical_name: String,
    pub checkers_search: String,
    pub pnp_search: String,
    pub expected_barcode: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BasketFile {
    basket: Vec<BasketItem>,
}

pub fn load(path: &str) -> Result<Vec<BasketItem>> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read basket file: {}", path))?;

    let parsed: BasketFile = toml::from_str(&content)
        .context("Failed to parse basket.toml")?;

    Ok(parsed.basket)
}
