use std::path::Path;
use std::sync::Arc;

use downloader::progress::Reporter;
use reqwest::Client;
use serde::Deserialize;
use tokio::fs;

use crate::filesystem::paths::GetUserGameDataDirError;
use crate::infra::archive::{extract_archive, ExtractionError};
use crate::infra::http_client::create_downloader;
use crate::infra::utils::OS;
use crate::settings::Settings;
use crate::soundpacks::models::{
    InstalledSoundpackMetadata, Soundpack, SoundpackCatalogEntry, SoundpackInstallStatus,
    SoundpacksForVariant, StockSoundpack, ThirdPartySoundpack,
};
use crate::soundpacks::repository::SoundpacksRepository;
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetSoundpacksForVariantsError {
    #[error("failed to get user game data directory: {0}")]
    UserGameDataDir(#[from] GetUserGameDataDirError),

    #[error("failed to read stock soundpacks: {0}")]
    ReadStockSoundpacks(#[from] ReadStockSoundpacksError),

    #[error("failed to access repository: {0}")]
    Repository(#[from] crate::soundpacks::repository::SoundpacksRepositoryError),
}

pub async fn get_soundpacks_for_variants(
    variants: &[GameVariant],
    os: &OS,
    data_dir: &Path,
    soundpack_catalog: &[SoundpackCatalogEntry],
    repository: &dyn SoundpacksRepository,
    active_release_repository: &dyn crate::active_release::repository::ActiveReleaseRepository,
) -> Result<Vec<SoundpacksForVariant>, GetSoundpacksForVariantsError> {
    let mut results = Vec::new();

    for variant in variants {
        let mut soundpacks = Vec::new();

        let active_version = variant
            .get_active_release(active_release_repository)
            .await
            .ok()
            .flatten();

        let stock_soundpacks = read_stock_soundpacks(variant, os, data_dir, active_version.as_deref())
            .await
            .ok()
            .unwrap_or_default();

        for stock in stock_soundpacks {
            soundpacks.push(Soundpack::Stock(stock));
        }

        for catalog_entry in soundpack_catalog {
            let installed_metadata = repository
                .get_installed_soundpack(variant, &catalog_entry.id)
                .await?;

            let (status, installed_last_updated_time) = match installed_metadata {
                Some(metadata) => (
                    SoundpackInstallStatus::Installed,
                    Some(metadata.installed_last_updated_time),
                ),
                None => (SoundpackInstallStatus::NotInstalled, None),
            };

            soundpacks.push(Soundpack::ThirdParty(ThirdPartySoundpack {
                id: catalog_entry.id.clone(),
                name: catalog_entry.name.clone(),
                description: catalog_entry.description.clone(),
                owner: catalog_entry.owner.clone(),
                repo: catalog_entry.repo.clone(),
                branch: catalog_entry.branch.clone(),
                internal_path: catalog_entry.internal_path.clone(),
                status,
                last_updated_time: None,
                installed_last_updated_time,
            }));
        }

        results.push(SoundpacksForVariant {
            variant: *variant,
            soundpacks,
        });
    }

    Ok(results)
}

#[derive(thiserror::Error, Debug)]
pub enum ReadStockSoundpacksError {
    #[error("failed to get game executable directory: {0}")]
    GameExecutableDir(#[from] crate::filesystem::paths::GetGameExecutableDirError),

    #[error("failed to read directory: {0}")]
    ReadDir(#[from] std::io::Error),
}

pub async fn read_stock_soundpacks(
    variant: &GameVariant,
    os: &OS,
    data_dir: &Path,
    active_version: Option<&str>,
) -> Result<Vec<StockSoundpack>, ReadStockSoundpacksError> {
    use crate::filesystem::paths::get_game_resources_dir;

    let active_version = match active_version {
        Some(version) => version,
        None => return Ok(Vec::new()),
    };

    let resources_dir = get_game_resources_dir(variant, &active_version, data_dir, os).await?;
    let sound_dir = resources_dir.join("data").join("sound");

    if !sound_dir.exists() {
        return Ok(Vec::new());
    }

    let mut soundpacks = Vec::new();
    let mut entries = fs::read_dir(&sound_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let soundpack_file = path.join("soundpack.txt");
        if !soundpack_file.exists() {
            continue;
        }

        if let Ok(content) = fs::read_to_string(&soundpack_file).await {
            let mut name = String::new();
            let mut view = String::new();

            for line in content.lines() {
                let line = line.trim();
                if line.starts_with("NAME ") {
                    name = line["NAME ".len()..].trim().to_string();
                } else if line.starts_with("VIEW ") {
                    view = line["VIEW ".len()..].trim().to_string();
                }
            }

            if !name.is_empty() {
                soundpacks.push(StockSoundpack {
                    name,
                    view: if view.is_empty() {
                        "Unknown".to_string()
                    } else {
                        view
                    },
                    path: path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                });
            }
        }
    }

    Ok(soundpacks)
}

#[derive(thiserror::Error, Debug)]
pub enum GetLastUpdatedTimeError {
    #[error("failed to fetch from GitHub: {0}")]
    Fetch(#[from] reqwest::Error),

    #[error("failed to parse timestamp: {0}")]
    Parse(#[from] chrono::ParseError),
}

#[derive(Deserialize)]
struct GitHubCommit {
    commit: GitHubCommitDetails,
}

#[derive(Deserialize)]
struct GitHubCommitDetails {
    author: GitHubCommitAuthor,
}

#[derive(Deserialize)]
struct GitHubCommitAuthor {
    date: String,
}

pub async fn get_last_updated_time(
    client: &Client,
    owner: &str,
    repo: &str,
    branch: &str,
) -> Result<chrono::DateTime<chrono::Utc>, GetLastUpdatedTimeError> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/commits/{}",
        owner, repo, branch
    );

    let response = client.get(&url).send().await?;
    response.error_for_status_ref()?;

    let commit: GitHubCommit = response.json().await?;
    let timestamp = chrono::DateTime::parse_from_rfc3339(&commit.commit.author.date)?;

    Ok(timestamp.with_timezone(&chrono::Utc))
}

#[derive(thiserror::Error, Debug)]
pub enum InstallSoundpackError<E: std::error::Error + Send + Sync + 'static> {
    #[error("failed to get user game data directory: {0}")]
    UserGameDataDir(#[from] GetUserGameDataDirError),

    #[error("failed to create downloader: {0}")]
    Downloader(#[from] downloader::Error),

    #[error("failed to download soundpack: {0}")]
    Download(#[from] reqwest::Error),

    #[error("failed to extract archive: {0}")]
    Extract(#[from] ExtractionError),

    #[error("failed to get last updated time: {0}")]
    LastUpdatedTime(#[from] GetLastUpdatedTimeError),

    #[error("failed to access repository: {0}")]
    Repository(#[from] crate::soundpacks::repository::SoundpacksRepositoryError),

    #[error("failed to perform IO operation: {0}")]
    Io(#[from] std::io::Error),

    #[error("status update callback failed: {0}")]
    Callback(E),

    #[error("soundpack source directory not found in extracted archive")]
    SourceDirNotFound,
}

pub async fn install_soundpack<E: std::error::Error + Send + Sync + 'static, F, Fut>(
    variant: &GameVariant,
    soundpack: &SoundpackCatalogEntry,
    client: &Client,
    os: &OS,
    data_dir: &Path,
    user_game_data_dir: &Path,
    settings: &Settings,
    _on_status_update: F,
    progress: Arc<dyn Reporter + Send + Sync>,
    repository: &dyn SoundpacksRepository,
) -> Result<(), InstallSoundpackError<E>>
where
    F: Fn(crate::soundpacks::models::SoundpackInstallStatusPayload) -> Fut,
    Fut: std::future::Future<Output = Result<(), E>> + Send,
{
    let temp_dir = data_dir.join("Temp").join("soundpacks");
    fs::create_dir_all(&temp_dir).await?;

    let download_url = format!(
        "https://github.com/{}/{}/archive/refs/heads/{}.zip",
        soundpack.owner, soundpack.repo, soundpack.branch
    );

    let download_path = temp_dir.join(format!("{}_{}.zip", soundpack.id, soundpack.branch));

    let mut downloader = create_downloader(client.clone(), &temp_dir, settings.parallel_requests)?;

    let download = downloader::Download::new(&download_url)
        .file_name(&download_path)
        .progress(progress);
    let results = downloader.async_download(&[download]).await?;

    if let Some(Err(e)) = results.into_iter().next() {
        return Err(InstallSoundpackError::Downloader(e));
    }

    let extract_dir = temp_dir.join(&soundpack.id);
    if extract_dir.exists() {
        fs::remove_dir_all(&extract_dir).await?;
    }
    fs::create_dir_all(&extract_dir).await?;

    extract_archive(&download_path, &extract_dir, os).await?;

    let mut source_dir = None;
    let mut entries = fs::read_dir(&extract_dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_dir() {
            source_dir = Some(path.join(&soundpack.internal_path));
            break;
        }
    }

    let source_dir = source_dir.ok_or(InstallSoundpackError::SourceDirNotFound)?;

    if !source_dir.exists() {
        return Err(InstallSoundpackError::SourceDirNotFound);
    }

    let sound_dir = user_game_data_dir.join("sound");
    fs::create_dir_all(&sound_dir).await?;

    let target_dir = sound_dir.join(&soundpack.id);
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).await?;
    }

    copy_dir_recursive(&source_dir, &target_dir).await?;

    let last_updated_time = get_last_updated_time(
        client,
        &soundpack.owner,
        &soundpack.repo,
        &soundpack.branch,
    )
    .await?;

    repository
        .save_installed_soundpack(&InstalledSoundpackMetadata {
            soundpack_id: soundpack.id.clone(),
            variant: *variant,
            installed_last_updated_time: last_updated_time,
        })
        .await?;

    let _ = fs::remove_file(&download_path).await;
    let _ = fs::remove_dir_all(&extract_dir).await;

    Ok(())
}

fn copy_dir_recursive<'a>(
    src: &'a Path,
    dst: &'a Path,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'a>> {
    Box::pin(async move {
        fs::create_dir_all(dst).await?;

        let mut entries = fs::read_dir(src).await?;
        while let Some(entry) = entries.next_entry().await? {
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                copy_dir_recursive(&src_path, &dst_path).await?;
            } else {
                fs::copy(&src_path, &dst_path).await?;
            }
        }

        Ok(())
    })
}

#[derive(thiserror::Error, Debug)]
pub enum UninstallSoundpackError {
    #[error("failed to get user game data directory: {0}")]
    UserGameDataDir(#[from] GetUserGameDataDirError),

    #[error("failed to access repository: {0}")]
    Repository(#[from] crate::soundpacks::repository::SoundpacksRepositoryError),

    #[error("failed to perform IO operation: {0}")]
    Io(#[from] std::io::Error),
}

pub async fn uninstall_soundpack(
    variant: &GameVariant,
    soundpack_id: &str,
    user_game_data_dir: &Path,
    repository: &dyn SoundpacksRepository,
) -> Result<(), UninstallSoundpackError> {
    let sound_dir = user_game_data_dir.join("sound");
    let target_dir = sound_dir.join(soundpack_id);

    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).await?;
    }

    repository
        .delete_installed_soundpack(variant, soundpack_id)
        .await?;

    Ok(())
}
