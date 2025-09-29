use std::error::Error;
use std::fs;
use std::path::Path;

use serde::de::DeserializeOwned;

use crate::variants::GameVariant;

pub fn get_safe_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

pub fn get_github_repo_for_variant(variant: &GameVariant) -> &'static str {
    match variant {
        GameVariant::DarkDaysAhead => "CleverRaven/Cataclysm-DDA",
        GameVariant::BrightNights => "cataclysmbnteam/Cataclysm-BN",
        GameVariant::TheLastGeneration => "Cataclysm-TLG/Cataclysm-TLG",
    }
}

pub fn read_from_file<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let v = serde_json::from_str(&contents)?;
    Ok(v)
}

pub fn write_to_file<T: serde::Serialize>(path: &Path, data: &T) -> Result<(), Box<dyn Error>> {
    let contents = serde_json::to_string_pretty(data)?;
    fs::write(path, contents)?;
    Ok(())
}
