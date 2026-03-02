use sqlx::PgPool;

use crate::{scrapers::checkers, storage::Repository};

mod api;
mod models;
mod normalizer;
mod scheduler;
mod scrapers;
mod storage;
mod transport;

#[tokio::main]
async fn main() {
    let client = transport::client::HttpClient::new();

    let checkers = checkers::Checkers::new(
        "https://www.checkers.co.za/api/catalogue/get-products-filter".to_string(),
        client,
    );

    let url = "postgres://username:password@host:port/db_name";
    let pool = PgPool::connect_lazy(url).unwrap();

    let repository = Repository::new(pool);
}
