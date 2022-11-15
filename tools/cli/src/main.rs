use clap::Parser;
use commands::{build, publish, Command};
use thiserror::Error;

mod commands;

#[derive(Parser)]
#[command(author, version, about)]
struct Opts {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    BuildError(#[from] build::Error),
    #[error(transparent)]
    PublishError(#[from] publish::Error),
}

fn main() -> Result<(), Error> {
    let opts = Opts::parse();
    if let Some(cmd) = opts.command {
        match cmd {
            Command::Build(cmd) => {
                build::run(cmd)?;
            }
            Command::Publish(cmd) => {
                publish::run(cmd)?;
            }
        }
    }
    Ok(())
}
