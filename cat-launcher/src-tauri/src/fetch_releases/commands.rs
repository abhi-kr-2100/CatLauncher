use serde::ser::SerializeStruct;
use serde::Serializer;
use strum_macros::IntoStaticStr;
use tauri::{command, AppHandle, Emitter, Manager};

use crate::fetch_releases::fetch_releases::{FetchReleasesError, ReleasesUpdatePayload};
use crate::infra::http_client::HTTP_CLIENT;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum FetchReleasesCommandError {
    #[error("system directory not found: {0}")]
    SystemDir(#[from] tauri::Error),

    #[error("failed to fetch releases: {0}")]
    Fetch(#[from] FetchReleasesError<tauri::Error>),
}

#[command]
pub async fn fetch_releases_for_variant(
    app_handle: AppHandle,
    variant: GameVariant,
) -> Result<(), FetchReleasesCommandError> {
    let cache_dir = app_handle.path().app_cache_dir()?;

    let on_releases = move |payload: ReleasesUpdatePayload| {
        app_handle.emit("releases-update", payload)?;
        Ok(())
    };

    variant
        .fetch_releases(&HTTP_CLIENT, &cache_dir, on_releases)
        .await?;

    Ok(())
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
