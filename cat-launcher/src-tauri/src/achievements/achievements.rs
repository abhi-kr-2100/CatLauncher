use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::active_release::repository::ActiveReleaseRepository;
use crate::filesystem::paths::{
  GetGameExecutableDirError, GetUserGameDataDirError,
  get_game_resources_dir, get_or_create_user_game_data_dir,
};
use crate::infra::utils::{OSNotSupportedError, get_os_enum};
use crate::variants::GameVariant;

use super::types::{Achievement, CharacterAchievements};

/// Represents the structure of an achievement file stored by the game.
#[derive(Serialize, Deserialize, Debug)]
pub struct AchievementFile {
  /// The version of the achievement format.
  pub achievement_version: i32,
  /// A list of achievement IDs earned.
  pub achievements: Vec<String>,
  /// The name of the character associated with these achievements.
  pub avatar_name: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum AchievementName {
  String(String),
  Object { str: String },
}

impl AchievementName {
  fn into_string(self) -> String {
    match self {
      AchievementName::String(s) => s,
      AchievementName::Object { str } => str,
    }
  }
}

#[derive(Deserialize)]
struct AchievementJsonEntry {
  id: String,
  #[serde(rename = "type")]
  entry_type: String,
  name: Option<AchievementName>,
}

/// Errors that can occur when retrieving achievements.
#[derive(thiserror::Error, Debug)]
pub enum GetAchievementsError {
  /// An error occurred while determining the user game data directory.
  #[error("failed to get user game data directory: {0}")]
  UserGameData(#[from] GetUserGameDataDirError),
  /// An error occurred while reading from the filesystem.
  #[error("failed to read achievements directory: {0}")]
  Read(#[from] std::io::Error),
  /// An error occurred while retrieving the active release.
  #[error("failed to get active release: {0}")]
  ActiveRelease(
    #[from] crate::active_release::active_release::ActiveReleaseError,
  ),
  /// The current operating system is not supported.
  #[error("unsupported OS: {0}")]
  UnsupportedOS(#[from] OSNotSupportedError),
  /// An error occurred while determining the game resources directory.
  #[error("failed to get game resources directory: {0}")]
  GameResourcesDir(#[from] GetGameExecutableDirError),
  /// An error occurred while parsing JSON data.
  #[error("failed to parse achievements.json: {0}")]
  Serde(#[from] serde_json::Error),
}

async fn load_achievement_names(
  variant: &GameVariant,
  data_dir: &Path,
  active_release_repository: &dyn ActiveReleaseRepository,
) -> Result<HashMap<String, String>, GetAchievementsError> {
  let Some(active_release) = variant
    .get_active_release(active_release_repository)
    .await?
  else {
    return Ok(HashMap::new());
  };

  let os = get_os_enum(std::env::consts::OS)?;
  let resources_dir =
    get_game_resources_dir(variant, &active_release, data_dir, &os)
      .await?;

  let achievements_json_path = resources_dir
    .join("data")
    .join("json")
    .join("achievements.json");

  let content =
    match tokio::fs::read_to_string(achievements_json_path).await {
      Ok(c) => c,
      Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
        return Ok(HashMap::new());
      }
      Err(e) => return Err(GetAchievementsError::Read(e)),
    };

  let entries =
    serde_json::from_str::<Vec<AchievementJsonEntry>>(&content)?;

  let id_to_name = entries
    .into_iter()
    .filter(|entry| entry.entry_type == "achievement")
    .filter_map(|entry| {
      entry.name.map(|name| (entry.id, name.into_string()))
    })
    .collect();

  Ok(id_to_name)
}

async fn load_character_achievement_ids(
  achievements_dir: PathBuf,
) -> Result<HashMap<String, HashSet<String>>, GetAchievementsError> {
  let mut dir = match tokio::fs::read_dir(achievements_dir).await {
    Ok(dir) => dir,
    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
      return Ok(HashMap::new());
    }
    Err(e) => return Err(GetAchievementsError::Read(e)),
  };

  let mut character_achievements_map: HashMap<
    String,
    HashSet<String>,
  > = HashMap::new();

  while let Some(entry) = dir.next_entry().await? {
    let path = entry.path();
    if path.extension().and_then(|s| s.to_str()) != Some("json") {
      continue;
    }

    let Ok(content) = tokio::fs::read_to_string(&path).await else {
      continue;
    };

    let Ok(achievement_file) =
      serde_json::from_str::<AchievementFile>(&content)
    else {
      continue;
    };

    character_achievements_map
      .entry(achievement_file.avatar_name)
      .or_default()
      .extend(achievement_file.achievements);
  }

  Ok(character_achievements_map)
}

/// Retrieves all achievements for a character across all game variants.
///
/// This function reads the achievement files from the user's game data directory
/// and maps them to their friendly names by parsing the game's achievement definitions.
pub async fn get_achievements(
  variant: &GameVariant,
  data_dir: &Path,
  active_release_repository: &dyn ActiveReleaseRepository,
) -> Result<Vec<CharacterAchievements>, GetAchievementsError> {
  let id_to_name = load_achievement_names(
    variant,
    data_dir,
    active_release_repository,
  )
  .await?;

  let user_data_dir =
    get_or_create_user_game_data_dir(variant, data_dir).await?;
  let achievements_dir = user_data_dir.join("achievements");

  let character_achievements_map =
    load_character_achievement_ids(achievements_dir).await?;

  let mut result: Vec<CharacterAchievements> =
    character_achievements_map
      .into_iter()
      .map(|(name, achievement_ids)| {
        let mut achievements: Vec<Achievement> = achievement_ids
          .into_iter()
          .map(|id| {
            let friendly_name =
              id_to_name.get(&id).unwrap_or(&id).to_string();
            Achievement {
              id,
              name: friendly_name,
            }
          })
          .collect();
        achievements.sort_by(|a, b| a.name.cmp(&b.name));
        CharacterAchievements {
          character_name: name,
          achievements,
        }
      })
      .collect();

  result.sort_by(|a, b| a.character_name.cmp(&b.character_name));

  Ok(result)
}

#[cfg(test)]
#[allow(
  clippy::panic_in_result_fn,
  clippy::indexing_slicing,
  clippy::expect_used,
  clippy::io_other_error,
  clippy::unwrap_used
)]
mod tests {
  use super::*;
  use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
  use crate::filesystem::paths::{
    get_game_resources_dir, get_or_create_asset_installation_dir,
    get_or_create_user_game_data_dir,
  };
  use crate::infra::testing::test_database::TestDatabase;
  use crate::infra::utils::get_os_enum;
  use tempfile::TempDir;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  #[tokio::test]
  async fn test_get_achievements_missing_directory() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      // Intentionally do NOT create achievements directory to test missing directory handling
      let result =
        get_achievements(&variant, temp_data.path(), &active_repo)
          .await?;

      assert!(result.is_empty());
    }
    Ok(())
  }

  #[tokio::test]
  async fn test_get_achievements_empty_directory() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      let ach_dir = user_data.join("achievements");
      tokio::fs::create_dir_all(&ach_dir).await?;

      let result =
        get_achievements(&variant, temp_data.path(), &active_repo)
          .await?;

      assert!(result.is_empty());
    }
    Ok(())
  }

  #[tokio::test]
  async fn test_get_achievements_no_active_release() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      let ach_dir = user_data.join("achievements");
      tokio::fs::create_dir_all(&ach_dir).await?;

      let ach_file = AchievementFile {
        achievement_version: 1,
        achievements: vec![
          "ACH_FIRST_KILL".to_string(),
          "ACH_SURVIVED_10_DAYS".to_string(),
        ],
        avatar_name: "Alice".to_string(),
      };
      let json_str = serde_json::to_string(&ach_file)?;
      tokio::fs::write(ach_dir.join("alice.json"), json_str).await?;

      let result =
        get_achievements(&variant, temp_data.path(), &active_repo)
          .await?;

      assert_eq!(result.len(), 1);
      assert_eq!(result[0].character_name, "Alice");
      assert_eq!(result[0].achievements.len(), 2);
      assert_eq!(result[0].achievements[0].name, "ACH_FIRST_KILL");
      assert_eq!(
        result[0].achievements[1].name,
        "ACH_SURVIVED_10_DAYS"
      );
    }

    Ok(())
  }

  #[tokio::test]
  async fn test_get_achievements_missing_definitions_directory()
  -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      let version = "v1.0.0";
      active_repo.set_active_release(&variant, version).await?;

      let install_dir = get_or_create_asset_installation_dir(
        &variant,
        version,
        temp_data.path(),
      )
      .await?;
      let game_dir = install_dir.join("cataclysm-dda");
      tokio::fs::create_dir_all(&game_dir).await?;

      // Intentionally do NOT create data/json/achievements.json directory or file to test missing definitions

      let user_data =
        get_or_create_user_game_data_dir(&variant, temp_data.path())
          .await?;
      let ach_dir = user_data.join("achievements");
      tokio::fs::create_dir_all(&ach_dir).await?;

      let ach_file = AchievementFile {
        achievement_version: 1,
        achievements: vec!["ACH_FIRST_KILL".to_string()],
        avatar_name: "Charlie".to_string(),
      };
      tokio::fs::write(
        ach_dir.join("charlie.json"),
        serde_json::to_string(&ach_file)?,
      )
      .await?;

      let result =
        get_achievements(&variant, temp_data.path(), &active_repo)
          .await?;

      assert_eq!(result.len(), 1);
      assert_eq!(result[0].character_name, "Charlie");
      assert_eq!(result[0].achievements.len(), 1);
      assert_eq!(result[0].achievements[0].name, "ACH_FIRST_KILL");
    }

    Ok(())
  }

  async fn run_achievements_with_definitions_case(
    db: &TestDatabase,
    temp_data: &TempDir,
    variant: GameVariant,
  ) -> TestResult {
    let active_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let version = "v1.0.0";

    // Set active release
    active_repo.set_active_release(&variant, version).await?;

    // Setup resources dir using helper function
    let install_dir = get_or_create_asset_installation_dir(
      &variant,
      version,
      temp_data.path(),
    )
    .await?;
    let game_dir = install_dir.join("cataclysm-dda");
    tokio::fs::create_dir_all(&game_dir).await?;

    let current_os = get_os_enum(std::env::consts::OS)?;
    let resources_dir = get_game_resources_dir(
      &variant,
      version,
      temp_data.path(),
      &current_os,
    )
    .await?;
    let json_dir = resources_dir.join("data").join("json");
    tokio::fs::create_dir_all(&json_dir).await?;

    let defs_json = r#"[
        {
          "id": "ACH_FIRST_KILL",
          "type": "achievement",
          "name": "First Blood"
        },
        {
          "id": "ACH_SURVIVED_10_DAYS",
          "type": "achievement",
          "name": { "str": "Decade of Survival" }
        }
      ]"#;
    tokio::fs::write(json_dir.join("achievements.json"), defs_json)
      .await?;

    // Setup user character achievements
    let user_data =
      get_or_create_user_game_data_dir(&variant, temp_data.path())
        .await?;
    let ach_dir = user_data.join("achievements");
    tokio::fs::create_dir_all(&ach_dir).await?;

    let ach_file = AchievementFile {
      achievement_version: 1,
      achievements: vec![
        "ACH_FIRST_KILL".to_string(),
        "ACH_SURVIVED_10_DAYS".to_string(),
      ],
      avatar_name: "Bob".to_string(),
    };
    tokio::fs::write(
      ach_dir.join("bob.json"),
      serde_json::to_string(&ach_file)?,
    )
    .await?;

    let result =
      get_achievements(&variant, temp_data.path(), &active_repo)
        .await?;

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].character_name, "Bob");
    assert_eq!(result[0].achievements.len(), 2);
    assert_eq!(result[0].achievements[0].name, "Decade of Survival");
    assert_eq!(result[0].achievements[1].name, "First Blood");

    Ok(())
  }

  #[tokio::test]
  async fn test_get_achievements_with_definitions() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let temp_data = TempDir::new()?;

    for variant in [
      GameVariant::DarkDaysAhead,
      GameVariant::BrightNights,
      GameVariant::TheLastGeneration,
    ] {
      run_achievements_with_definitions_case(
        &db, &temp_data, variant,
      )
      .await?;
    }

    Ok(())
  }
}
