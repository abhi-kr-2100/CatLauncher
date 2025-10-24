use tauri::{command, AppHandle, Manager, State};

use crate::game_tips::error::CommandError;
use crate::game_tips::game_tips::get_all_tips_for_variant;
use crate::infra::utils::get_os_enum;
use crate::repository::sqlite_last_played_repository::SqliteLastPlayedVersionRepository;
use crate::variants::GameVariant;

#[command]
pub async fn get_tips(
    app_handle: AppHandle,
    variant: GameVariant,
    last_played_repository: State<'_, SqliteLastPlayedVersionRepository>,
) -> Result<Vec<String>, CommandError> {
    let data_dir = app_handle.path().app_local_data_dir()?;
    let os = get_os_enum(std::env::consts::OS)?;

    let tips = get_all_tips_for_variant(&variant, &data_dir, &os, &*last_played_repository).await?;
    Ok(tips)
}
