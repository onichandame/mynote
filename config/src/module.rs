use std::{env, error::Error, sync::Arc};

/// a globally accessible configuration provider
pub type ConfigProvider = Arc<Config>;

pub struct Config {
    /// env: DATABASE_URL
    pub database_url: String,
    /// env: SERVER_ADDR
    pub server_addr: String,
    pub mode: Mode,
}

/// Execution mode
pub enum Mode {
    UnitTest,
    Production,
}

/// constructor
pub fn new_config_provider(mode: Mode) -> Result<ConfigProvider, Box<dyn Error + Send + Sync>> {
    Ok(Arc::new(match mode {
        Mode::Production => Config {
            mode,
            server_addr: env::var("SERVER_ADDR")?,
            database_url: env::var("DATABASE_URL")?,
        },
        Mode::UnitTest => Config {
            mode,
            database_url: "sqlite://:memory:".to_owned(),
            server_addr: "".to_owned(),
        },
    }))
}
