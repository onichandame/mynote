use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author,version,about,long_about=None)]
pub struct Config {
    #[clap(short, long, value_parser, env, default_value = "sqlite::memory:")]
    pub database_url: String,
    #[clap(short, long, value_parser, env, default_value = "amqp://127.0.0.1:5672")]
    pub pubsub_url:String,
    #[clap(short, long, env, default_value = "80")]
    pub port: u16,
    #[clap(long, short = 'o')]
    pub allow_origins: Vec<String>,
    #[clap(skip = "api")]
    pub api_path: String,
    #[clap(skip = "health")]
    pub health_path: String,
    #[clap(long, env)]
    pub cdn_api_secret: String,
    #[clap(long, env)]
    pub cdn_api_key: String,
    #[clap(skip = "notebook")]
    pub cdn_content_folder: String,
    /// If set to true, new users are only allowed to sign up when a valid invitation key is provided
    #[clap(long, env, value_parser, default_value_t = false)]
    pub invitation_only: bool,
}

impl Config {
    pub fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
