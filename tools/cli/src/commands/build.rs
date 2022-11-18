use std::{
    io,
    process::{Command, Stdio},
};

use clap::Parser;
use thiserror::Error;

use crate::package::Package;

#[derive(Parser)]
pub struct Build {}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    ValueParseError(#[from] serde_yaml::Error),
}

pub fn run(pkgs: &Vec<Package>) -> Result<(), Error> {
    for pkg in pkgs {
        build_package(pkg)?;
    }
    Ok(())
}

fn build_package(pkg: &Package) -> Result<(), Error> {
    Command::new("docker")
        .args([
            "build",
            ".",
            "-t",
            &pkg.get_image_tag(),
            "--build-arg",
            &format!("PKG={}", &pkg.name),
            "--build-arg",
            &format!("PKG_ROOT={}", &pkg.path),
            "-f",
            &format!("docker/{}.dockerfile", &pkg.config.build_script),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}
