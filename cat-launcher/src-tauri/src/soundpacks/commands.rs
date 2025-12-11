use std::env::consts::OS;
use std::sync::Arc;

use strum::IntoStaticStr;
use tauri::ipc::Channel;
use tauri::{command, AppHandle, Emitter, Manager, State};

use cat_macros::CommandErrorSerialize;

use crate::filesystem::paths::{get_or_create_user_game_data_dir, GetUserGameDataDirError};
use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
use crate::infra::http_client::HTTP_CLIENT;
use crate::infra::utils::{get_os_enum, OSNotSupportedError};
use crate::install_release::channel_reporter::ChannelReporter;
use crate::settings::Settings;
use crate::soundpacks::models::{
    SoundpackCatalogEntry, SoundpackInstallProgressStatus, SoundpackInstallStatusPayload,
    SoundpacksForVariant,
};
use crate::soundpacks::repository::sqlite_soundpacks_repository::SqliteSoundpacksRepository;
use crate::soundpacks::soundpacks::{
    get_last_updated_time, get_soundpacks_for_variants, install_soundpack, uninstall_soundpack,
    GetLastUpdatedTimeError, GetSoundpacksForVariantsError, InstallSoundpackError,
    UninstallSoundpackError,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize)]
pub enum GetSoundpacksForVariantsCommandError {
    #[error("failed to get soundpacks: {0}")]
    GetSoundpacks(#[from] GetSoundpacksForVariantsError),

    #[error("failed to get OS enum: {0}")]
    Os(#[from] OSNotSupportedError),

    #[error("system directory not found: {0}")]
    SystemDir(#[from] tauri::Error),
}

#[command]
pub async fn get_soundpacks_for_variants_command(
    app_handle: AppHandle,
    variants: Vec<GameVariant>,
    repository: State<'_, SqliteSoundpacksRepository>,
    active_release_repository: State<'_, SqliteActiveReleaseRepository>,
    settings: State<'_, Settings>,
) -> Result<Vec<SoundpacksForVariant>, GetSoundpacksForVariantsCommandError> {
    let data_dir = app_handle.path().app_local_data_dir()?;
    let os = get_os_enum(OS)?;

    let result = get_soundpacks_for_variants(
        &variants,
        &os,
        &data_dir,
        &settings.soundpacks,
        &*repository,
        &*active_release_repository,
    )
    .await?;

    Ok(result)
}

#[derive(thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize)]
pub enum GetLastUpdatedTimeCommandError {
    #[error("failed to get last updated time: {0}")]
    GetLastUpdatedTime(#[from] GetLastUpdatedTimeError),
}

#[command]
pub async fn get_last_updated_time_command(
    soundpack: SoundpackCatalogEntry,
) -> Result<String, GetLastUpdatedTimeCommandError> {
    let timestamp = get_last_updated_time(
        &HTTP_CLIENT,
        &soundpack.owner,
        &soundpack.repo,
        &soundpack.branch,
    )
    .await?;

    Ok(timestamp.to_rfc3339())
}

#[derive(thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize)]
pub enum InstallSoundpackCommandError {
    #[error("failed to install soundpack: {0}")]
    Install(#[from] InstallSoundpackError<tauri::Error>),

    #[error("failed to get OS enum: {0}")]
    Os(#[from] OSNotSupportedError),

    #[error("system directory not found: {0}")]
    SystemDir(#[from] tauri::Error),

    #[error("failed to get user game data directory: {0}")]
    UserGameDataDir(#[from] GetUserGameDataDirError),
}

#[command]
pub async fn install_soundpack_command(
    app_handle: AppHandle,
    variant: GameVariant,
    soundpack: SoundpackCatalogEntry,
    repository: State<'_, SqliteSoundpacksRepository>,
    settings: State<'_, Settings>,
    on_download_progress: Channel,
) -> Result<(), InstallSoundpackCommandError> {
    let data_dir = app_handle.path().app_local_data_dir()?;
    let os = get_os_enum(OS)?;
    let user_game_data_dir = get_or_create_user_game_data_dir(&variant, &data_dir).await?;

    let soundpack_id = soundpack.id.clone();

    let on_status_update = {
        let app_handle = app_handle.clone();
        move |payload: SoundpackInstallStatusPayload| {
            let app_handle = app_handle.clone();
            async move { app_handle.emit("soundpack-install-status", payload) }
        }
    };

    let progress = Arc::new(ChannelReporter::new(on_download_progress));

    on_status_update(SoundpackInstallStatusPayload {
        status: SoundpackInstallProgressStatus::Downloading,
        variant,
        soundpack_id: soundpack_id.clone(),
    })
    .await
    .map_err(InstallSoundpackError::Callback)?;

    install_soundpack(
        &variant,
        &soundpack,
        &HTTP_CLIENT,
        &os,
        &data_dir,
        &user_game_data_dir,
        &settings,
        {
            let app_handle = app_handle.clone();
            let soundpack_id = soundpack_id.clone();
            move |_| {
                let app_handle = app_handle.clone();
                let soundpack_id = soundpack_id.clone();
                async move {
                    app_handle.emit(
                        "soundpack-install-status",
                        SoundpackInstallStatusPayload {
                            status: SoundpackInstallProgressStatus::Installing,
                            variant,
                            soundpack_id,
                        },
                    )
                }
            }
        },
        progress,
        &*repository,
    )
    .await?;

    on_status_update(SoundpackInstallStatusPayload {
        status: SoundpackInstallProgressStatus::Success,
        variant,
        soundpack_id,
    })
    .await
    .map_err(InstallSoundpackError::Callback)?;

    Ok(())
}

#[derive(thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize)]
pub enum UninstallSoundpackCommandError {
    #[error("failed to uninstall soundpack: {0}")]
    Uninstall(#[from] UninstallSoundpackError),

    #[error("system directory not found: {0}")]
    SystemDir(#[from] tauri::Error),

    #[error("failed to get user game data directory: {0}")]
    UserGameDataDir(#[from] GetUserGameDataDirError),
}

#[command]
pub async fn uninstall_soundpack_command(
    app_handle: AppHandle,
    variant: GameVariant,
    soundpack_id: String,
    repository: State<'_, SqliteSoundpacksRepository>,
) -> Result<(), UninstallSoundpackCommandError> {
    let data_dir = app_handle.path().app_local_data_dir()?;
    let user_game_data_dir = get_or_create_user_game_data_dir(&variant, &data_dir).await?;

    let on_status_update = {
        let app_handle = app_handle.clone();
        move |payload: SoundpackInstallStatusPayload| {
            let app_handle = app_handle.clone();
            async move { app_handle.emit("soundpack-install-status", payload) }
        }
    };

    on_status_update(SoundpackInstallStatusPayload {
        status: SoundpackInstallProgressStatus::Installing,
        variant,
        soundpack_id: soundpack_id.clone(),
    })
    .await
    .map_err(|e: tauri::Error| UninstallSoundpackError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    uninstall_soundpack(&variant, &soundpack_id, &user_game_data_dir, &*repository).await?;

    on_status_update(SoundpackInstallStatusPayload {
        status: SoundpackInstallProgressStatus::Success,
        variant,
        soundpack_id: soundpack_id.clone(),
    })
    .await
    .map_err(|e: tauri::Error| UninstallSoundpackError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

    Ok(())
}
