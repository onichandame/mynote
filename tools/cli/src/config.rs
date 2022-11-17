use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub build_script: String,
}
