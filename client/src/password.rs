use crud::BooleanFilter;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamPasswordOutput {
    pub stream_passwords: model::password::Model,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PasswordFilter {
    pub is_local: Option<BooleanFilter>,
}
