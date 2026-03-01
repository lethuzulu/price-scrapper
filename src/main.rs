use crate::scrapers::checkers;

mod api;
mod models;
mod normalizer;
mod scheduler;
mod scrapers;
mod transport;

#[tokio::main]
async fn main() {
    let client = transport::client::HttpClient::new();


    let checkers = checkers::Checkers::new(
        "https://www.checkers.co.za/api/catalogue/get-products-filter".to_string(),
        client,
    );

    if let Err(e) = checkers.search("milk").await {
        println!("error  {}", e)
    }
}
