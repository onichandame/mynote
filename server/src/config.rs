use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about=None)]
pub struct Config {
    #[clap(short, long, value_parser, default_value = "sqlite::memory:")]
    pub database_url: String,
    #[clap(short, long, default_value = "80")]
    pub port: u16,
    #[clap(long, short = 'o')]
    pub allow_origins: Vec<String>,
    #[clap(
        long,
        short = 'c',
        default_value = "content",
        help = "Root directory of contents. Contents are plain files stored locally"
    )]
    pub content_dir: String,
    #[clap(skip = "content")]
    pub content_root: String,
    #[clap(skip = "api")]
    pub api_root: String,
}

impl Config {
    pub fn validate(&self) -> Result<(), Box<dyn Error + Sync + Send>> {
        if self.api_root == self.content_root {
            return Err("content and api cannot share the same root path".into());
        }
        Ok(())
    }
}
