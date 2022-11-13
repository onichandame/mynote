use std::{
    fs, io,
    process::{Command, Stdio},
};

use clap::Parser;
use thiserror::Error;

static PKG_ROOT: &str = "apps";

#[derive(Parser)]
pub struct Build {
    /// Choose which package to build. If not provided, all packages will be built
    #[clap(short = 'p', long, env)]
    package: Option<Vec<String>>,
    /// Specify the name of the script to be run for this build
    #[clap(short = 'f', long, env)]
    file: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error("build error: {0}")]
    Unknown(String),
}

pub fn run(cmd: Build) -> Result<(), Error> {
    let packages = cmd.package.unwrap_or(get_all_packages()?);
    for pkg in packages {
        build_package(&pkg, &cmd.file);
    }
    Ok(())
}

fn build_package(package: &str, file: &str) {
    Command::new("docker")
        .args([
            "build",
            ".",
            "-t",
            &package,
            "--build-arg",
            &format!("PKG={}", &package),
            "-f",
            file,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
}

fn get_all_packages() -> Result<Vec<String>, Error> {
    let mut pkgs = vec![];
    for subfile in fs::read_dir(PKG_ROOT)? {
        if let Ok(subfile) = subfile {
            if subfile.file_type()?.is_dir() {
                pkgs.push(subfile.file_name().into_string().map_err(|_| {
                    Error::Unknown("a package's path contains non UTF-8 characters".to_owned())
                })?);
            }
        }
    }
    Ok(pkgs)
}
