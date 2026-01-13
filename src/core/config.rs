// Configuration module
// Handles loading and validating application configuration

pub struct Config {
    pub api_port: u16,
    pub database_url: String,
    pub ml_api_url: String,
    pub optimization_interval: u64,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        // TODO: Load from TOML and environment variables
        Ok(Config {
            api_port: 6701,
            database_url: "sqlite:///data/depin-orcha.db".to_string(),
            ml_api_url: "http://localhost:6702".to_string(),
            optimization_interval: 900, // 15 minutes
        })
    }
}
