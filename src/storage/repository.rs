use sqlx::{PgPool, query};
use chrono::{Utc};
use crate::models::Product;
use anyhow::Result;

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

        for product in products { // todo: consider using a single batched insert instead of a loop
            let scrapped_at = product.scraped_at.unwrap_or_else(Utc::now);
            query(
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
            .bind(scrapped_at)
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
