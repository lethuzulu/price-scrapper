use anyhow::Result;
use chrono::Utc;
use sqlx::PgPool;

use crate::models::Product;

pub mod repository;

pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_product(&self, products: &[Product]) -> Result<()> {
        if products.is_empty() {
            return Ok(());
        }

        let mut tx = self.pool.begin().await?;

        for product in products {
            let scraped_at = product.scraped_at.unwrap_or_else(Utc::now);

            sqlx::query(
                r#"
                INSERT INTO price_records (name, price, retailer, sku, barcode, scraped_at)
                VALUES ($1, $2, $3, $4, $5, $6)
                "#,
            )
            .bind(&product.name)
            .bind(product.price)
            .bind(&product.retailer)
            .bind(&product.sku)
            .bind(product.barcode.as_ref())
            .bind(scraped_at)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn get_latest_price(&self) {
        todo!("implement later")
    }

    pub async fn get_price_history(&self) {
        todo!("implement later")
    }
}
