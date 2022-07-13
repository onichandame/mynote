use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author,version,about,long_about=None)]
pub struct Args {
    #[clap(short, long, value_parser, default_value = "sqlite::memory:")]
    pub database_url: String,
    #[clap(short, long, default_value = "127.0.0.1:80")]
    pub addr: String,
}
