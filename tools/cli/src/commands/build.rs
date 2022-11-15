use std::{
    fs, io,
    process::{Command, Stdio},
};

use clap::Parser;
use serde::Deserialize;
use thiserror::Error;

static PKG_ROOT: &str = "apps";
static CONFIG_PATH: &str = "build.yaml";

#[derive(Parser)]
pub struct Build {
    /// Choose which packages to build. If not provided, all packages will be built
    #[clap(short = 'p', long, env)]
    package: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct Config {
    script: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    ValueParseError(#[from] serde_yaml::Error),
    #[error("build error: {0}")]
    Unknown(String),
}

pub fn run(cmd: Build) -> Result<(), Error> {
    let packages = get_all_packages()?
        .into_iter()
        .filter(|(pkg, _)| {
            if let Some(pkgs) = cmd.package.as_ref() {
                if pkgs.iter().any(|v| v == pkg) {
                    true
                } else {
                    false
                }
            } else {
                true
            }
        })
        .collect::<Vec<_>>();
    for (pkg, config) in packages {
        build_package(&pkg, &config);
    }
    Ok(())
}

fn build_package(package: &str, config: &Config) {
    Command::new("docker")
        .args([
            "build",
            ".",
            "-t",
            &package,
            "--build-arg",
            &format!("PKG={}", &package),
            "-f",
            &format!("docker/{}.dockerfile", &config.script),
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
}

fn get_all_packages() -> Result<Vec<(String, Config)>, Error> {
    let mut pkgs = vec![];
    for subfile in fs::read_dir(PKG_ROOT)? {
        if let Ok(subfile) = subfile {
            if subfile.file_type()?.is_dir() {
                let config_path = subfile.path().join(CONFIG_PATH);
                if config_path.exists() {
                    let config = serde_yaml::from_str::<Config>(&fs::read_to_string(config_path)?)?;
                    pkgs.push((
                        subfile.file_name().into_string().map_err(|_| {
                            Error::Unknown(
                                "a package's path contains non UTF-8 characters".to_owned(),
                            )
                        })?,
                        config,
                    ));
                }
            }
        }
    }
    Ok(pkgs)
}
