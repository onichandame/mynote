use std::{
    io,
    process::{Command, Stdio},
};

use clap::Parser;
use thiserror::Error;

use crate::package::Package;

#[derive(Parser)]
pub struct Publish {
    /// The remote container registry where the images will be pushed
    #[clap(short = 'r', long, env, default_value = "docker.io")]
    registry: String,
    /// The username of the container registry
    #[clap(short = 'u', long, env)]
    username: String,
    /// The password of the container registry
    #[clap(short = 'p', long, env)]
    password: Option<String>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
}

pub fn run(opts: Publish, pkgs: &Vec<Package>) -> Result<(), Error> {
    if let Some(password) = opts.password.as_ref() {
        login_registry(&opts.registry, &opts.username, password)?;
    }
    for pkg in pkgs {
        publish_package(pkg, &opts)?;
    }
    Ok(())
}

fn login_registry(registry: &str, username: &str, password: &str) -> Result<(), Error> {
    Command::new("docker")
        .args([
            "login",
            &format!("--username={}", username),
            &format!("--password={}", password),
            registry,
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}

fn publish_package(pkg: &Package, opts: &Publish) -> Result<(), Error> {
    let full_name = format!(
        "{}/{}/{}",
        opts.registry,
        opts.username,
        pkg.get_image_tag()
    );
    Command::new("docker")
        .args(["push", &full_name])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}
