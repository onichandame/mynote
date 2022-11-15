use clap::Parser;
use thiserror::Error;

#[derive(Parser)]
pub struct Publish {}

#[derive(Error, Debug)]
pub enum Error {}

pub fn run(cmd: Publish) -> Result<(), Error> {
    Ok(())
}
