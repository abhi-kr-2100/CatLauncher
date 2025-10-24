use std::collections::HashMap;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::filesystem::paths::get_last_played_filepath;
use crate::infra::utils::{read_from_file, write_to_file};
use crate::repository::last_played_repository::{
    LastPlayedVersionRepository, LastPlayedVersionRepositoryError,
};
use crate::variants::game_variant::GameVariant;

#[derive(Debug, Serialize, Deserialize)]
pub struct LastPlayedData {
    pub versions: HashMap<String, String>,
}

pub struct FileLastPlayedVersionRepository {
    data_dir: PathBuf,
}

impl FileLastPlayedVersionRepository {
    pub fn new(data_dir: &Path) -> Self {
        Self {
            data_dir: data_dir.to_path_buf(),
        }
    }
}

#[async_trait]
impl LastPlayedVersionRepository for FileLastPlayedVersionRepository {
    async fn get_last_played_version(
        &self,
        game_variant: &GameVariant,
    ) -> Result<Option<String>, LastPlayedVersionRepositoryError> {
        let path = get_last_played_filepath(game_variant, &self.data_dir)
            .await
            .map_err(|err| LastPlayedVersionRepositoryError::Get(Box::new(err)))?;

        match fs::metadata(&path).await {
            Ok(metadata) if metadata.is_file() => {}
            _ => return Ok(None),
        };

        let mut data = read_from_file::<LastPlayedData>(&path)
            .await
            .map_err(|err| LastPlayedVersionRepositoryError::Get(Box::new(err)))?;
        let variant_key: &'static str = game_variant.into();

        Ok(data.versions.remove(variant_key))
    }

    async fn set_last_played_version(
        &self,
        game_variant: &GameVariant,
        version: &str,
    ) -> Result<(), LastPlayedVersionRepositoryError> {
        let path = get_last_played_filepath(game_variant, &self.data_dir)
            .await
            .map_err(|err| LastPlayedVersionRepositoryError::Set(Box::new(err)))?;

        let mut data = match fs::metadata(&path).await {
            Ok(metadata) if metadata.is_file() => read_from_file(&path)
                .await
                .map_err(|err| LastPlayedVersionRepositoryError::Set(Box::new(err)))?,
            _ => LastPlayedData {
                versions: std::collections::HashMap::new(),
            },
        };

        let variant_key: &'static str = game_variant.into();

        data.versions
            .insert(variant_key.into(), version.to_string());

        write_to_file(&path, &data)
            .await
            .map_err(|err| LastPlayedVersionRepositoryError::Set(Box::new(err)))?;

        Ok(())
    }
}
