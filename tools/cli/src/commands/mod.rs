use clap::Subcommand;

use self::build::Build;

pub mod build;

#[derive(Subcommand)]
pub enum Command {
    /// Build package(s) into docker images
    Build(Build),
}
