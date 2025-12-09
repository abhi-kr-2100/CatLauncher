use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::fetch_releases::fetch_releases::{ReleasesUpdatePayload, ReleasesUpdateStatus};
use crate::fetch_releases::repository::ReleasesRepository;
use crate::filesystem::paths::get_default_releases_file_path;
use crate::game_release::game_release::GameRelease;
use crate::game_release::utils::gh_release_to_game_release;
use crate::infra::github::asset::GitHubAsset;
use crate::infra::github::release::GitHubRelease;
use crate::infra::utils::read_from_file;
use crate::variants::GameVariant;

pub async fn get_default_releases(
    variant: &GameVariant,
    resources_dir: &Path,
) -> Vec<GitHubRelease> {
    let default_releases_file = get_default_releases_file_path(variant, resources_dir);
    if !default_releases_file.is_file() {
        return Vec::new();
    }

    match read_from_file::<Vec<GitHubRelease>>(&default_releases_file).await {
        Ok(releases) => releases,
        _ => Vec::new(),
    }
}

pub fn merge_releases(r1: &[GitHubRelease], r2: &[GitHubRelease]) -> Vec<GitHubRelease> {
    let mut map: HashMap<u64, GitHubRelease> = HashMap::with_capacity(r1.len() + r2.len());

    for release in r1.iter().chain(r2.iter()) {
        map.entry(release.id)
            .and_modify(|existing_release| {
                let assets_capacity = existing_release.assets.len() + release.assets.len();
                let mut seen_asset_ids = HashSet::with_capacity(assets_capacity);
                let mut new_assets = Vec::with_capacity(assets_capacity);

                for asset in existing_release.assets.drain(..) {
                    if seen_asset_ids.insert(asset.id) {
                        new_assets.push(asset);
                    }
                }

                for asset in release.assets.iter() {
                    if seen_asset_ids.insert(asset.id) {
                        new_assets.push(asset.clone());
                    }
                }

                existing_release.assets = new_assets;
            })
            .or_insert_with(|| release.clone());
    }

    map.into_values().collect()
}

pub async fn get_assets(
    release: &GameRelease,
    resources_dir: &Path,
    releases_repository: &dyn ReleasesRepository,
) -> Vec<GitHubAsset> {
    let cached_releases = releases_repository
        .get_cached_releases(&release.variant)
        .await
        .unwrap_or_default(); // It's okay if cached releases couldn't be read.
    let default_releases = get_default_releases(&release.variant, resources_dir).await;
    let all_releases = merge_releases(&cached_releases, &default_releases);

    let maybe_release = all_releases.iter().find(|r| r.tag_name == release.version);

    if let Some(release) = maybe_release {
        release.assets.clone()
    } else {
        Vec::new()
    }
}

pub fn get_releases_payload(
    variant: &GameVariant,
    gh_releases: &[GitHubRelease],
    status: ReleasesUpdateStatus,
) -> ReleasesUpdatePayload {
    let mut releases: Vec<GameRelease> = gh_releases
        .iter()
        .map(|r| gh_release_to_game_release(r, variant))
        .collect();

    releases.sort_by(|a, b| b.created_at.cmp(&a.created_at));

    ReleasesUpdatePayload {
        variant: *variant,
        releases,
        status,
    }
}
