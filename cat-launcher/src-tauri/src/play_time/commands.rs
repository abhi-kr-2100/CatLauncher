use serde::ser::SerializeStruct;
use serde::Serialize;
use strum::IntoStaticStr;
use tauri::State;

use crate::play_time::play_time::{
  get_play_time_for_variant as get_play_time_for_variant_feature,
  get_play_time_for_version as get_play_time_for_version_feature,
};
use crate::play_time::repository::PlayTimeRepositoryError;
use crate::play_time::sqlite_play_time_repository::SqlitePlayTimeRepository;
use crate::variants::game_variant::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum GetPlayTimeCommandError {
  #[error("Failed to get play time: {0}")]
  Repository(#[from] PlayTimeRepositoryError),
}

impl Serialize for GetPlayTimeCommandError {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut st =
      serializer.serialize_struct("GetPlayTimeCommandError", 2)?;
    let err_type: &'static str = self.into();
    st.serialize_field("type", &err_type)?;
    let msg = self.to_string();
    st.serialize_field("message", &msg)?;
    st.end()
  }
}

#[tauri::command]
pub async fn get_play_time_for_variant(
  variant: GameVariant,
  repository: State<'_, SqlitePlayTimeRepository>,
) -> Result<i64, GetPlayTimeCommandError> {
  let result =
    get_play_time_for_variant_feature(&variant, &*repository).await?;
  Ok(result)
}

#[tauri::command]
pub async fn get_play_time_for_version(
  variant: GameVariant,
  version: String,
  repository: State<'_, SqlitePlayTimeRepository>,
) -> Result<i64, GetPlayTimeCommandError> {
  let result = get_play_time_for_version_feature(
    &variant,
    &version,
    &*repository,
  )
  .await?;
  Ok(result)
}
