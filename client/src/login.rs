use serde::{Deserialize, Serialize};

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginInput {
    pub identity: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginOutput {
    pub login: String,
}
