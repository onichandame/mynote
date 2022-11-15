use clap::Subcommand;

use self::{build::Build, publish::Publish};

pub mod build;
pub mod publish;

#[derive(Subcommand)]
pub enum Command {
    /// Build package(s) into docker images. Packages are found in the `apps` directory and only those containg a `build.yaml` configuration file will be built.
    Build(Build),
    Publish(Publish),
}
