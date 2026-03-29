use std::io;
use std::path::Path;
use tokio::fs::{read_dir, File};
use tokio::io::AsyncReadExt;

use crate::filesystem::paths::{
  get_or_create_user_game_data_dir, GetUserGameDataDirError,
};
use crate::variants::GameVariant;
use crate::world_options::schema::{get_known_options, OptionType};
use crate::world_options::types::{World, WorldOption};

#[derive(thiserror::Error, Debug)]
pub enum ListWorldsError {
  #[error("failed to get user game data directory: {0}")]
  UserDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to read save directory: {0}")]
  ReadSaveDir(io::Error),
}

pub async fn list_worlds(
  variant: &GameVariant,
  data_dir: &Path,
) -> Result<Vec<World>, ListWorldsError> {
  let user_data_dir =
    get_or_create_user_game_data_dir(variant, data_dir).await?;
  let save_dir = user_data_dir.join("save");

  if !save_dir.exists() {
    return Ok(vec![]);
  }

  let mut worlds = vec![];
  let mut entries = read_dir(save_dir)
    .await
    .map_err(ListWorldsError::ReadSaveDir)?;

  while let Some(entry) = entries
    .next_entry()
    .await
    .map_err(ListWorldsError::ReadSaveDir)?
  {
    if entry
      .file_type()
      .await
      .map_err(ListWorldsError::ReadSaveDir)?
      .is_dir()
    {
      let world_dir = entry.path();
      if world_dir.join("worldoptions.json").exists() {
        worlds.push(World {
          name: entry.file_name().to_string_lossy().into_owned(),
        });
      }
    }
  }

  Ok(worlds)
}

#[derive(thiserror::Error, Debug)]
pub enum GetWorldOptionsError {
  #[error("failed to get user game data directory: {0}")]
  UserDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to read world options file: {0}")]
  Read(io::Error),

  #[error("failed to parse world options file: {0}")]
  Parse(#[from] serde_json::Error),
}

pub async fn get_world_options(
  variant: &GameVariant,
  world_name: &str,
  data_dir: &Path,
) -> Result<Vec<WorldOption>, GetWorldOptionsError> {
  let user_data_dir =
    get_or_create_user_game_data_dir(variant, data_dir).await?;
  let options_path = user_data_dir
    .join("save")
    .join(world_name)
    .join("worldoptions.json");

  let mut file = File::open(options_path)
    .await
    .map_err(GetWorldOptionsError::Read)?;
  let mut content = String::new();
  file
    .read_to_string(&mut content)
    .await
    .map_err(GetWorldOptionsError::Read)?;

  let options: Vec<WorldOption> = serde_json::from_str(&content)?;
  Ok(options)
}

#[derive(thiserror::Error, Debug)]
pub enum UpdateWorldOptionsError {
  #[error("failed to get user game data directory: {0}")]
  UserDataDir(#[from] GetUserGameDataDirError),

  #[error("failed to write world options file: {0}")]
  Write(io::Error),

  #[error("failed to serialize world options: {0}")]
  Serialize(#[from] serde_json::Error),

  #[error("validation failed for option '{name}': {message}")]
  Validation { name: String, message: String },
}

pub async fn update_world_options(
  variant: &GameVariant,
  world_name: &str,
  options: Vec<WorldOption>,
  data_dir: &Path,
) -> Result<(), UpdateWorldOptionsError> {
  // Validate options
  let known_options = get_known_options();
  for option in &options {
    if let Some(schema) =
      known_options.iter().find(|s| s.name == option.name)
    {
      match &schema.option_type {
        OptionType::Boolean => {
          if option.value != "true" && option.value != "false" {
            return Err(UpdateWorldOptionsError::Validation {
              name: option.name.clone(),
              message: "must be 'true' or 'false'".to_string(),
            });
          }
        }
        OptionType::Integer => {
          let val = option
            .value
            .trim_end_matches('%')
            .parse::<f64>()
            .map_err(|_| UpdateWorldOptionsError::Validation {
              name: option.name.clone(),
              message: "must be an integer".to_string(),
            })?;
          if let Some(min) = schema.min {
            if val < min {
              return Err(UpdateWorldOptionsError::Validation {
                name: option.name.clone(),
                message: format!("must be at least {}", min),
              });
            }
          }
          if let Some(max) = schema.max {
            if val > max {
              return Err(UpdateWorldOptionsError::Validation {
                name: option.name.clone(),
                message: format!("must be at most {}", max),
              });
            }
          }
        }
        OptionType::Float => {
          let val = option.value.parse::<f64>().map_err(|_| {
            UpdateWorldOptionsError::Validation {
              name: option.name.clone(),
              message: "must be a float".to_string(),
            }
          })?;
          if let Some(min) = schema.min {
            if val < min {
              return Err(UpdateWorldOptionsError::Validation {
                name: option.name.clone(),
                message: format!("must be at least {}", min),
              });
            }
          }
          if let Some(max) = schema.max {
            if val > max {
              return Err(UpdateWorldOptionsError::Validation {
                name: option.name.clone(),
                message: format!("must be at most {}", max),
              });
            }
          }
        }
        OptionType::Enum(values) => {
          if !values.contains(&option.value) {
            return Err(UpdateWorldOptionsError::Validation {
              name: option.name.clone(),
              message: format!(
                "must be one of: {}",
                values.join(", ")
              ),
            });
          }
        }
        OptionType::String => {}
      }
    } else if option.name.starts_with("SPAWN_RATE_") {
      // Validate spawn rates as floats
      let val = option.value.parse::<f64>().map_err(|_| {
        UpdateWorldOptionsError::Validation {
          name: option.name.clone(),
          message: "must be a float".to_string(),
        }
      })?;
      if !(0.0..=20.0).contains(&val) {
        return Err(UpdateWorldOptionsError::Validation {
          name: option.name.clone(),
          message: "must be between 0.0 and 20.0".to_string(),
        });
      }
    }
  }

  let user_data_dir =
    get_or_create_user_game_data_dir(variant, data_dir).await?;
  let options_path = user_data_dir
    .join("save")
    .join(world_name)
    .join("worldoptions.json");

  let content = serde_json::to_string_pretty(&options)?;
  tokio::fs::write(options_path, content)
    .await
    .map_err(UpdateWorldOptionsError::Write)?;

  Ok(())
}
