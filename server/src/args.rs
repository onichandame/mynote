use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about=None)]
pub struct Args {
    #[clap(short, long, value_parser, default_value = "sqlite::memory:")]
    pub database_url: String,
    #[clap(short, long, default_value = "80")]
    pub port: u16,
    #[clap(long)]
    pub allow_origins: Vec<String>,
    #[clap(
        long,
        default_value = "content",
        help = "Root directory of contents. Contents are plain files stored locally"
    )]
    pub content_dir: String,
    #[clap(
        long,
        default_value = "content",
        help = "Root path from which the contents are served"
    )]
    pub content_path: String,
    #[clap(
        long,
        default_value = "graphql",
        help = "Endpoint from which the graphql APIs are served"
    )]
    pub api_path: String,
}

impl Args {
    pub fn validate(&self) -> Result<(), Box<dyn Error + Sync + Send>> {
        if self.content_path == self.api_path {
            return Err("Endpoints for content and api must be different".into());
        }
        Ok(())
    }
}
