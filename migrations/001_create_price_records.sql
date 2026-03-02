CREATE TABLE price_records (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    price DECIMAL(10,2) NOT NULL,
    retailer TEXT NOT NULL,
    sku TEXT NOT NULL,
    barcode TEXT,
    scraped_at TIMESTAMPTZ NOT NULL
)

CREATE INDEX idx_price_records_retailer_sku ON price_records(retailer, sku);
CREATE INDEX idx_price_records_scraped_at ON price_records(scraped_at);

