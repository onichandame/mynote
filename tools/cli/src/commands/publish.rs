use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct Publish {
    #[clap(short = 'p', long, env)]
    packages: Option<Vec<String>>,
}

#[derive(Error, Debug)]
pub enum Error {}

pub fn run() -> Result<(), Error> {
    Ok(())
}
