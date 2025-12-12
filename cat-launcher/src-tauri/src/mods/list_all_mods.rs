use std::path::Path;

use thiserror::Error;
use tokio::fs;

use crate::active_release::active_release::ActiveReleaseError;
use crate::active_release::repository::ActiveReleaseRepository;
use crate::filesystem::paths::{get_game_resources_dir, GetGameExecutableDirError};
use crate::infra::utils::OS;
use crate::mods::mod_info::{Mod, ModInfoJson, ModInfoJsonEntry, ModStatus, ModType, ModsJson};
use crate::mods::repository::{InstalledModsRepository, InstalledModsRepositoryError};
use crate::variants::GameVariant;

#[derive(Error, Debug)]
pub enum LoadThirdPartyModsError {
    #[error("failed to read mods.json: {0}")]
    IoError(#[from] std::io::Error),

    #[error("failed to parse mods.json: {0}")]
    ParseError(#[from] serde_json::Error),
}

async fn load_third_party_mods(
    variant: &GameVariant,
    installed_mod_ids: &[String],
    resource_dir: &Path,
) -> Result<Vec<Mod>, LoadThirdPartyModsError> {
    let mods_json_path = resource_dir.join("mods.json");
    let mods_json_content = fs::read_to_string(mods_json_path).await?;

    let mods_data: ModsJson = serde_json::from_str(&mods_json_content)?;

    let Some(mods) = mods_data.get(variant.id()) else {
        return Ok(Vec::new());
    };

    let third_party_mods = mods
        .iter()
        .map(|mod_entry| {
            let status = if installed_mod_ids.contains(&mod_entry.id) {
                ModStatus::Installed
            } else {
                ModStatus::NotInstalled
            };

            Mod {
                mod_type: ModType::ThirdParty,
                id: mod_entry.id.clone(),
                name: mod_entry.name.clone(),
                description: mod_entry.description.clone(),
                category: mod_entry.category.clone(),
                status,
            }
        })
        .collect();

    Ok(third_party_mods)
}

#[derive(Error, Debug)]
pub enum LoadStockModsError {
    #[error("failed to get game resources directory: {0}")]
    ResourcesDir(#[from] GetGameExecutableDirError),
}

async fn parse_modinfo_json(modinfo_path: &Path) -> Option<ModInfoJson> {
    let content = fs::read_to_string(modinfo_path).await.ok()?;
    serde_json::from_str(&content).ok()
}

fn mod_entry_to_mod(entry: ModInfoJsonEntry) -> Mod {
    Mod {
        mod_type: ModType::Stock,
        id: entry.id,
        name: entry.name,
        description: entry.description,
        category: entry.category,
        status: ModStatus::Installed,
    }
}

async fn collect_mods_from_dir(mod_dir: &Path) -> Vec<Mod> {
    let modinfo_path = mod_dir.join("modinfo.json");
    let modinfo_exists = fs::metadata(&modinfo_path)
        .await
        .map(|_| true)
        .unwrap_or(false);

    if !modinfo_exists {
        return Vec::new();
    }

    let Some(modinfo_json) = parse_modinfo_json(&modinfo_path).await else {
        return Vec::new();
    };

    modinfo_json.into_iter().map(mod_entry_to_mod).collect()
}

async fn load_stock_mods(
    variant: &GameVariant,
    release_version: &str,
    data_dir: &Path,
    os: &OS,
) -> Result<Vec<Mod>, LoadStockModsError> {
    let resources_dir = get_game_resources_dir(variant, release_version, data_dir, os).await?;
    let mods_dir = resources_dir.join("data").join("mods");

    let mods_dir_exists = fs::metadata(&mods_dir)
        .await
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false);

    if !mods_dir_exists {
        return Ok(Vec::new());
    }

    let Ok(mut mod_entries) = fs::read_dir(&mods_dir).await else {
        return Ok(Vec::new());
    };

    let mut stock_mods = Vec::new();

    while let Ok(Some(entry)) = mod_entries.next_entry().await {
        if let Ok(file_type) = entry.file_type().await {
            if file_type.is_dir() {
                let mods_from_dir = collect_mods_from_dir(&entry.path()).await;
                stock_mods.extend(mods_from_dir);
            }
        }
    }

    Ok(stock_mods)
}

#[derive(Error, Debug)]
#[error("no active release found for variant")]
pub struct NoActiveReleaseError;

#[derive(Error, Debug)]
pub enum ListAllModsError {
    #[error("failed to get active release: {0}")]
    ActiveRelease(#[from] ActiveReleaseError),

    #[error("no active release: {0}")]
    NoActiveRelease(#[from] NoActiveReleaseError),

    #[error("failed to get installed mods: {0}")]
    InstalledMods(#[from] InstalledModsRepositoryError),

    #[error("failed to load third-party mods: {0}")]
    ThirdPartyMods(#[from] LoadThirdPartyModsError),

    #[error("failed to load stock mods: {0}")]
    StockMods(#[from] LoadStockModsError),
}

pub async fn list_all_mods(
    data_dir: &Path,
    resource_dir: &Path,
    variant: &GameVariant,
    os: &OS,
    active_release_repository: &dyn ActiveReleaseRepository,
    installed_mods_repository: &dyn InstalledModsRepository,
) -> Result<Vec<Mod>, ListAllModsError> {
    let release_version = variant
        .get_active_release(active_release_repository)
        .await?
        .ok_or(NoActiveReleaseError)?;

    let installed_mod_ids = installed_mods_repository.get_all_installed_mods(variant).await?;

    let mut all_mods = Vec::new();

    let third_party_mods = load_third_party_mods(variant, &installed_mod_ids, resource_dir).await?;
    all_mods.extend(third_party_mods);

    let stock_mods = load_stock_mods(variant, &release_version, data_dir, os).await?;
    all_mods.extend(stock_mods);

    Ok(all_mods)
}
