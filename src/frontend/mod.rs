use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "frontend/dist/"]
pub struct Frontend;

#[cfg(test)]
mod tests {
    use super::Frontend;

    #[test]
    fn files_loaded() -> Result<(), String> {
        let mut index_loaded = false;
        let mut assets_loaded = false;
        for asset in Frontend::iter() {
            if asset.starts_with("assets") {
                assets_loaded = true
            }
            if asset.eq("index.html") {
                index_loaded = true
            }
        }
        if !index_loaded {
            Err(String::from("index not loaded"))
        } else if !assets_loaded {
            Err(String::from("assets not loaded"))
        } else {
            Ok(())
        }
    }
}
