pub struct Config {
    pub database: DatabaseConfig,
}

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
}

impl Config {
    pub fn load() {}
}
