use crate::{config::Config, constants::IMAGE_NAME};

pub struct Package {
    pub name: String,
    pub path: String,
    pub config: Config,
}

impl Package {
    pub fn get_image_tag(&self) -> String {
        format!("{}:{}", IMAGE_NAME, &self.name)
    }
}
