use async_graphql::{Context, Object, Result};
use serde::Serialize;
use sha1::{Digest, Sha1};

use crate::{config::Config, entity};

#[derive(Default)]
pub struct ContentQuery {}

#[derive(Serialize)]
pub struct ContentUploadParams {
    pub signature: String,
    pub folder: String,
    pub timestamp: i64,
    pub api_key: String,
}

#[Object]
impl ContentQuery {
    /// Get post parameters for one upload action in JSON string. Normally includes a one-time token
    async fn upload_params(&self, ctx: &Context<'_>) -> Result<String> {
        let config = ctx.data::<Config>()?;
        ctx.data::<entity::user::Model>()?;
        let params = ContentUploadParams::new(&config.cdn_api_key, &config.cdn_content_folder)
            .sign(&config.cdn_api_secret);
        Ok(serde_json::to_string(&params)?)
    }
}

impl Default for ContentUploadParams {
    fn default() -> Self {
        Self {
            api_key: "".to_owned(),
            timestamp: chrono::Utc::now().naive_utc().timestamp(),
            folder: "".to_owned(),
            signature: "".to_owned(),
        }
    }
}

impl ContentUploadParams {
    pub fn new(api_key: &str, folder: &str) -> Self {
        let mut params = Self::default();
        params.folder = folder.to_owned();
        params.api_key = api_key.to_owned();
        params
    }
    fn sign(mut self, secret: &str) -> Self {
        // append secret
        let formatted_str = format!(
            "folder={}&timestamp={}{}",
            &self.folder, &self.timestamp, secret
        );
        let mut hasher = Sha1::new();
        hasher.update(formatted_str);
        self.signature = format!("{:X}", hasher.finalize());
        self
    }
}
