use std::{
    io,
    process::{Command, Stdio},
};

use clap::Parser;
use thiserror::Error;

use crate::config::Config;

#[derive(Parser)]
pub struct Build {}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    ValueParseError(#[from] serde_yaml::Error),
}

pub fn run(pkgs: &Vec<(String, String, Config)>) -> Result<(), Error> {
    for (name, path, config) in pkgs {
        build_package(name, path, config);
    }
    Ok(())
}

fn build_package(name: &str, path: &str, config: &Config) {
    Command::new("docker")
        .args([
            "build",
            ".",
            "-t",
            &name,
            "--build-arg",
            &format!("PKG={}", &name),
            "--build-arg",
            &format!("PKG_ROOT={}", &path),
            "-f",
            &format!("docker/{}.dockerfile", &config.build_script),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
}
