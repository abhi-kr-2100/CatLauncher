use serde::ser::SerializeStruct;
use serde::Serializer;
use strum_macros::IntoStaticStr;
use tauri::{command, AppHandle, Manager};

use crate::fetch_releases::fetch_releases::FetchReleasesError;
use crate::game_release::GameRelease;
use crate::infra::http_client::HTTP_CLIENT;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum FetchReleasesCommandError {
    #[error("no cache directory found: {0}")]
    NoCacheDir(#[from] tauri::Error),

    #[error("failed to fetch releases: {0}")]
    Fetch(#[from] FetchReleasesError),
}

#[command]
pub async fn fetch_releases_for_variant(
    app_handle: AppHandle,
    variant: GameVariant,
) -> Result<Vec<GameRelease>, FetchReleasesCommandError> {
    let cache_dir = app_handle.path().app_cache_dir()?;

    Ok(variant.fetch_releases(&HTTP_CLIENT, &cache_dir).await?)
}

impl serde::Serialize for FetchReleasesCommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("FetchReleasesCommandError", 2)?;

        let err_type: &'static str = self.into();
        st.serialize_field("type", &err_type)?;

        let msg = self.to_string();
        st.serialize_field("message", &msg)?;

        st.end()
    }
}
