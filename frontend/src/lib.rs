use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "spa/dist/"]
pub struct Spa;

#[cfg(test)]
mod tests {
    use crate::Spa;

    #[test]
    fn files_loaded() -> Result<(), String> {
        let mut index_loaded = false;
        let mut assets_loaded = false;
        for asset in Spa::iter() {
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
