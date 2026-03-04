use anyhow::Result;
use sqlx::{PgPool, migrate};

use crate::{scrapers::{checkers, pnp}, storage::Repository};

mod api;
mod config;
mod models;
mod normalizer;
mod scheduler;
mod scrapers;
mod storage;
mod transport;

#[tokio::main]
async fn main() -> Result<()> {
    let client = transport::client::HttpClient::new();

    let checkers = checkers::Checkers::new(
        "https://www.checkers.co.za/api/catalogue/get-products-filter".to_string(),
        client.clone(),
    );

    let pnp = pnp::Pnp::new(
        "https://www.pnp.co.za/pnphybris/v2/pnp-spa/products/suggestions".to_string(),
        client.clone(),
        "WC21".to_string(),
    );

    pnp.search("milk").await?;

    // let r = migrate!("./migrations").run(&pool).await?;

    // let repository = Repository::new(pool);
    Ok(())
}
