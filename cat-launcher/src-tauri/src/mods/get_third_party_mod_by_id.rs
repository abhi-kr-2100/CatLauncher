use std::path::Path;
use crate::infra::utils::Asset;
use crate::mods::list_all_mods::{self, ListThirdPartyModsError};
use crate::mods::types::{Mod, ThirdPartyMod};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetThirdPartyModByIdError {
    #[error("failed to list third party mods: {0}")]
    ListThirdPartyMods(#[from] ListThirdPartyModsError),
    #[error("mod not found")]
    ModNotFound,
}

pub async fn get_third_party_mod_by_id(
    mod_id: &str,
    variant: &GameVariant,
    resource_dir: &Path,
) -> Result<ThirdPartyMod, GetThirdPartyModByIdError> {
    let mods = list_all_mods::list_all_third_party_mods(variant, resource_dir).await?;
    let mod_data = mods.iter().find(|m| m.id() == mod_id);

    if mod_data.is_none() {
        return Err(GetThirdPartyModByIdError::ModNotFound);
    }

    match mod_data.unwrap() {
        Mod::ThirdParty(m) => Ok(m.clone()),
        _ => Err(GetThirdPartyModByIdError::ModNotFound),
    }
}
