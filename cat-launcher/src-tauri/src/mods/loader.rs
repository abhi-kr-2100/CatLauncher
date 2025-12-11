use std::collections::BTreeMap;
use std::path::Path;
use std::str::FromStr;

use serde::Deserialize;

use crate::mods::models::ThirdPartyMod;
use crate::variants::GameVariant;

#[derive(Debug, Deserialize)]
struct ModsCatalog {
    #[serde(flatten)]
    entries: BTreeMap<String, Vec<ThirdPartyModRecord>>,
}

#[derive(Debug, Deserialize)]
struct ThirdPartyModRecord {
    pub id: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub maintainers: Vec<String>,
    pub category: String,
    pub repository: String,
}

#[derive(thiserror::Error, Debug)]
pub enum LoadThirdPartyModsError {
    #[error("failed to read mods catalog: {0}")]
    Read(#[from] std::io::Error),

    #[error("failed to parse mods catalog: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("unknown variant '{0}' in mods catalog")]
    UnknownVariant(String),
}

/// Loads the `mods.json` catalog located in the Tauri resource directory.
///
/// The file is structured as a JSON object with the variant identifier as the key
/// (e.g. `"DarkDaysAhead"`, `"BrightNights"`, `"TheLastGeneration"`). Each key maps
/// to an array of third-party mod objects that contain the fields `id`, `name`,
/// `description`, `authors`, `maintainers`, `category`, and `repository`.
///
/// ```json
/// {
///   "DarkDaysAhead": [ { "id": "dda-aftershock", ... } ],
///   "BrightNights": [ { "id": "bn-dark-skies", ... } ],
///   "TheLastGeneration": [ { "id": "tlg-solstice", ... } ]
/// }
/// ```
///
/// This helper normalises the shape into a flat list of [`ThirdPartyMod`] values by
/// attaching the corresponding [`GameVariant`] to each entry.
pub fn load_third_party_mods(
    mods_catalog_path: &Path,
) -> Result<Vec<ThirdPartyMod>, LoadThirdPartyModsError> {
    let contents = std::fs::read_to_string(mods_catalog_path)?;
    let catalog: ModsCatalog = serde_json::from_str(&contents)?;

    let mut mods = Vec::new();

    for (variant_key, entries) in catalog.entries {
        let variant = GameVariant::from_str(&variant_key)
            .map_err(|_| LoadThirdPartyModsError::UnknownVariant(variant_key.clone()))?;

        for entry in entries {
            mods.push(ThirdPartyMod {
                id: entry.id,
                name: entry.name,
                description: entry.description,
                authors: entry.authors,
                maintainers: entry.maintainers,
                category: entry.category,
                repository: entry.repository,
                variant,
                status: None,
            });
        }
    }

    Ok(mods)
}
