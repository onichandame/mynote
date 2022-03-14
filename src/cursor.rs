use std::{error::Error, str};

use base64::{decode, encode};

#[derive(Default)]
pub struct Cursor {
    pub offset: u64,
}

impl Cursor {
    pub fn parse(string: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self {
            offset: str::from_utf8(&decode(string)?)?.parse()?,
        })
    }
}

impl ToString for Cursor {
    fn to_string(&self) -> String {
        encode(self.offset.to_string().as_bytes())
    }
}
