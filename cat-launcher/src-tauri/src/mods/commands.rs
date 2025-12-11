use std::collections::HashMap;

use chrono::Utc;
use serde::ser::SerializeStruct;
use serde::Serialize;
use strum::IntoStaticStr;
use tauri::{AppHandle, Manager, State};

use crate::mods::loader::{load_third_party_mods, LoadThirdPartyModsError};
use crate::mods::models::{ThirdPartyMod, ThirdPartyModStatus};
use crate::mods::repository::installed_mods_repository::{
    InstalledModsRepository, InstalledModsRepositoryError,
};
use crate::mods::repository::sqlite_installed_mods_repository::SqliteInstalledModsRepository;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum ListThirdPartyModsCommandError {
    #[error("failed to locate resource directory: {0}")]
    ResourceDir(#[from] tauri::Error),

    #[error("failed to load third-party mods catalog: {0}")]
    Load(#[from] LoadThirdPartyModsError),

    #[error("failed to fetch installed mods: {0}")]
    InstalledMods(#[from] InstalledModsRepositoryError),
}

impl Serialize for ListThirdPartyModsCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ListThirdPartyModsCommandError", 2)?;
        let error_type: &str = self.into();
        state.serialize_field("type", error_type)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

#[tauri::command]
pub async fn list_third_party_mods_for_variant(
    variant: GameVariant,
    app_handle: AppHandle,
    installed_mods_repository: State<'_, SqliteInstalledModsRepository>,
) -> Result<Vec<ThirdPartyMod>, ListThirdPartyModsCommandError> {
    let resource_dir = app_handle.path().resource_dir()?;
    let mods_catalog_path = resource_dir.join("mods.json");
    let mods = load_third_party_mods(&mods_catalog_path).await?;

    let mut filtered: Vec<ThirdPartyMod> = mods
        .into_iter()
        .filter(|mod_entry| mod_entry.variant == variant)
        .collect();

    let mut statuses: HashMap<String, ThirdPartyModStatus> = installed_mods_repository
        .list_installed_mods(&variant)
        .await?
        .into_iter()
        .map(|status| (status.mod_id.clone(), status))
        .collect();

    for mod_entry in &mut filtered {
        if let Some(status) = statuses.remove(&mod_entry.id) {
            mod_entry.status = Some(status);
        }
    }

    Ok(filtered)
}

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum ModifyInstalledModCommandError {
    #[error("failed to update installed mods: {0}")]
    Repository(#[from] InstalledModsRepositoryError),
}

impl Serialize for ModifyInstalledModCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ModifyInstalledModCommandError", 2)?;
        let error_type: &str = self.into();
        state.serialize_field("type", error_type)?;
        state.serialize_field("message", &self.to_string())?;
        state.end()
    }
}

#[tauri::command]
pub async fn mark_third_party_mod_installed(
    variant: GameVariant,
    mod_id: String,
    installed_mods_repository: State<'_, SqliteInstalledModsRepository>,
) -> Result<ThirdPartyModStatus, ModifyInstalledModCommandError> {
    let status = ThirdPartyModStatus {
        variant,
        mod_id,
        installed_at: Utc::now().to_rfc3339(),
        last_updated_time: None,
    };

    installed_mods_repository
        .upsert_installed_mod(&status)
        .await?;

    Ok(status)
}

#[tauri::command]
pub async fn remove_third_party_mod_installation(
    variant: GameVariant,
    mod_id: String,
    installed_mods_repository: State<'_, SqliteInstalledModsRepository>,
) -> Result<(), ModifyInstalledModCommandError> {
    installed_mods_repository
        .delete_installed_mod(&variant, &mod_id)
        .await?;

    Ok(())
}
