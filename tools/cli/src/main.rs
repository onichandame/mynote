use std::{fs, io};

use clap::Parser;
use commands::{build, publish, Command};
use config::Config;
use constants::{CONFIG_PATH, PKG_ROOT};
use package::Package;
use thiserror::Error;

mod commands;
mod config;
mod constants;
mod package;

/// Cli tool to build/publish the apps
#[derive(Parser)]
#[command(author, version, about)]
struct CliMain {
    /// The relevent package(app)s involved
    #[clap(short = 'p', long, env)]
    packages: Option<Vec<String>>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Error, Debug)]
enum Error {
    #[error(transparent)]
    BuildError(#[from] build::Error),
    #[error(transparent)]
    PublishError(#[from] publish::Error),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    ValueParseError(#[from] serde_yaml::Error),
    #[error("build error: {0}")]
    Unknown(String),
}

fn main() -> Result<(), Error> {
    let opts = CliMain::parse();
    let pkgs = get_all_packages()?
        .into_iter()
        .filter(|pkg| {
            if let Some(pkgs) = opts.packages.as_ref() {
                pkgs.contains(&pkg.name)
            } else {
                true
            }
        })
        .collect::<Vec<_>>();
    if let Some(cmd) = opts.command {
        match cmd {
            Command::Build(_) => {
                build::run(&pkgs)?;
            }
            Command::Publish(opts) => {
                publish::run(opts, &pkgs)?;
            }
        }
    }
    Ok(())
}

fn get_all_packages() -> Result<Vec<Package>, Error> {
    let mut pkgs = vec![];
    for subfile in fs::read_dir(PKG_ROOT)? {
        if let Ok(subdir) = subfile {
            if subdir.file_type()?.is_dir() {
                let config_path = subdir.path().join(CONFIG_PATH);
                if config_path.exists() {
                    let config = serde_yaml::from_str::<Config>(&fs::read_to_string(config_path)?)?;
                    let package_name = subdir.file_name().into_string().map_err(|_| {
                        Error::Unknown("a package's name contains non UTF-8 characters".to_owned())
                    })?;
                    pkgs.push(Package {
                        name: package_name,
                        path: subdir
                            .path()
                            .to_str()
                            .ok_or(Error::Unknown(
                                "a package's path contains non UTF-8 characters".to_owned(),
                            ))?
                            .to_owned(),
                        config,
                    });
                }
            }
        }
    }
    Ok(pkgs)
}
