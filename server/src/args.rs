use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about=None)]
pub struct Args {
    #[clap(short, long, value_parser, default_value = "sqlite::memory:")]
    pub database_url: String,
    #[clap(short, long, default_value = "80")]
    pub port: u16,
    #[clap(short, long)]
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
}
