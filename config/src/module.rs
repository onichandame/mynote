use std::{env, error::Error};

pub struct ConfigModule {
    pub database_url: String,
    /// the address on which the server listens
    pub server_addr: String,
    pub mode: Mode,
}

/// constructor
impl ConfigModule {
    pub fn create(mode: Mode) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(match mode {
            Mode::Production => Self {
                mode,
                server_addr: env::var("SERVER_ADDR")?,
                database_url: env::var("DATABASE_URL")?,
            },
            Mode::UnitTest => Self {
                mode,
                database_url: "sqlite://:memory:".to_owned(),
                server_addr: "127.0.0.1:3000".to_owned(),
            },
        })
    }
}

/// Execution mode
pub enum Mode {
    UnitTest,
    Production,
}
