use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::path::Path;

use tokio::fs::{read_dir, read_to_string};

use crate::active_release::repository::ActiveReleaseRepository;
use crate::infra::http_client::HttpClient;
use crate::infra::utils::{OS, sort_assets};
use crate::mods::lib::{
  GetStockModsDirError, get_mods_resource_path, get_stock_mods_dir,
};
use crate::mods::online::types::OnlineModRepository;
use crate::mods::repository::mods_repository::{
  ListCachedThirdPartyModsError, ModsRepository,
  SaveThirdPartyModsError,
};
use crate::mods::types::{
  FetchOnlineModsError, Mod, ModsUpdatePayload, ModsUpdateStatus,
  StockMod, ThirdPartyMod,
};
use crate::variants::GameVariant;

#[derive(thiserror::Error, Debug)]
pub enum GetAllStockModsError {
  #[error("failed to get active release: {0}")]
  GetActiveRelease(Box<dyn std::error::Error + Send + Sync>),

  #[error("failed to get stock mods dir: {0}")]
  GetStockModsDir(#[from] GetStockModsDirError),

  #[error("failed to read stock mod dir: {0}")]
  ListStockMods(#[from] ListAllStockModsError),
}

pub async fn get_all_stock_mods(
  game_variant: &GameVariant,
  data_dir: &Path,
  os: &OS,
  active_release_repository: &impl ActiveReleaseRepository,
) -> Result<Vec<StockMod>, GetAllStockModsError> {
  let release_version = active_release_repository
    .get_active_release(game_variant)
    .await
    .map_err(|e| {
      GetAllStockModsError::GetActiveRelease(Box::new(e))
    })?;

  if let Some(release_version) = release_version {
    let stock_mods_dir = get_stock_mods_dir(
      game_variant,
      &release_version,
      data_dir,
      os,
    )
    .await?;

    let stock_mods = list_all_stock_mods(&stock_mods_dir).await?;
    Ok(stock_mods)
  } else {
    Ok(Vec::new())
  }
}

#[derive(thiserror::Error, Debug)]
pub enum ListAllModsError<E: std::error::Error> {
  #[error("failed to get stock mods dir: {0}")]
  GetStockModsDir(#[from] GetStockModsDirError),

  #[error("failed to read stock mod dir: {0}")]
  ExtractStockMod(#[from] ListAllStockModsError),

  #[error("failed to get active release: {0}")]
  GetActiveRelease(Box<dyn std::error::Error + Send + Sync>),

  #[error("failed to list third-party mods: {0}")]
  ListThirdPartyMods(#[from] ListThirdPartyModsError),

  #[error("failed to send update: {0}")]
  Send(E),

  #[error("failed to fetch online mods: {0}")]
  OnlineFetch(#[from] FetchOnlineModsError),

  #[error("failed to save mods to repository: {0}")]
  Repository(#[from] SaveThirdPartyModsError),

  #[error("failed to list cached third-party mods: {0}")]
  ListCachedMods(#[from] ListCachedThirdPartyModsError),
}

#[allow(clippy::too_many_arguments)]
pub async fn list_all_mods<F, E>(
  game_variant: &GameVariant,
  data_dir: &Path,
  resource_dir: &Path,
  os: &OS,
  active_release_repository: &impl ActiveReleaseRepository,
  mods_repository: &impl ModsRepository,
  online_mod_repositories: &[Box<dyn OnlineModRepository>],
  client: &impl HttpClient,
  on_update: F,
) -> Result<(), ListAllModsError<E>>
where
  E: std::error::Error,
  F: Fn(ModsUpdatePayload) -> Result<(), E>,
{
  let mut all_mods = Vec::new();
  let mut mod_ids = HashSet::new();

  let cached_mods =
    list_cached_third_party_mods(game_variant, mods_repository)
      .await
      .map_err(ListAllModsError::ListCachedMods)?;

  for cached_mod in cached_mods {
    mod_ids.insert(cached_mod.id.clone());
    all_mods.push(Mod::ThirdParty(cached_mod));
  }

  sort_assets(&mut all_mods);

  on_update(ModsUpdatePayload {
    variant: *game_variant,
    mods: all_mods.clone(),
    status: ModsUpdateStatus::Fetching,
  })
  .map_err(ListAllModsError::Send)?;

  for repo in online_mod_repositories {
    let online_mods =
      repo.get_mods_for_variant(game_variant, client).await?;

    let mut new_mods_added = false;
    for tp_mod in online_mods {
      if mod_ids.insert(tp_mod.id.clone()) {
        all_mods.push(Mod::ThirdParty(tp_mod));
        new_mods_added = true;
      }
    }

    if new_mods_added {
      sort_assets(&mut all_mods);

      on_update(ModsUpdatePayload {
        variant: *game_variant,
        mods: all_mods.clone(),
        status: ModsUpdateStatus::Fetching,
      })
      .map_err(ListAllModsError::Send)?;
    }
  }

  let local_third_party_mods =
    list_all_third_party_mods(game_variant, resource_dir)
      .await
      .map_err(ListAllModsError::ListThirdPartyMods)?;

  for tp in local_third_party_mods {
    if mod_ids.insert(tp.id.clone()) {
      all_mods.push(Mod::ThirdParty(tp));
    }
  }

  let stock_mods = get_all_stock_mods(
    game_variant,
    data_dir,
    os,
    active_release_repository,
  )
  .await
  .map_err(|e| ListAllModsError::GetActiveRelease(Box::new(e)))?;

  for sp in stock_mods {
    all_mods.push(Mod::Stock(sp));
  }

  sort_assets(&mut all_mods);

  mods_repository
    .save_third_party_mods(
      game_variant,
      all_mods
        .iter()
        .filter_map(|m| {
          if let Mod::ThirdParty(tp) = m {
            Some(tp.clone())
          } else {
            None
          }
        })
        .collect(),
    )
    .await?;

  on_update(ModsUpdatePayload {
    variant: *game_variant,
    mods: all_mods,
    status: ModsUpdateStatus::Success,
  })
  .map_err(ListAllModsError::Send)?;

  Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum ExtractStockModError {
  #[error("failed to parse modinfo.json: {0}")]
  ParseModInfo(#[from] serde_json::Error),

  #[error("no MOD_INFO entry found")]
  NoModInfo,

  #[error("modinfo missing required field: {0}")]
  MissingField(&'static str),
}

fn extract_stock_mod_from_modinfo(
  modinfo_content: &str,
) -> Result<StockMod, ExtractStockModError> {
  let entries: Vec<serde_json::Value> =
    serde_json::from_str(modinfo_content)?;

  let mod_info = entries
    .iter()
    .find(|entry| {
      entry.get("type").and_then(|t| t.as_str()) == Some("MOD_INFO")
    })
    .ok_or(ExtractStockModError::NoModInfo)?;

  let id = mod_info
    .get("id")
    .and_then(|i| i.as_str())
    .ok_or(ExtractStockModError::MissingField("id"))?
    .to_string();

  let name = mod_info
    .get("name")
    .and_then(|n| n.as_str())
    .ok_or(ExtractStockModError::MissingField("name"))?
    .to_string();

  let description = mod_info
    .get("description")
    .and_then(|d| d.as_str())
    .unwrap_or("")
    .to_string();

  let category = mod_info
    .get("category")
    .and_then(|c| c.as_str())
    .unwrap_or("")
    .to_string();

  Ok(StockMod {
    id,
    name,
    description,
    category,
  })
}

#[derive(Debug, thiserror::Error)]
pub enum ListAllStockModsError {
  #[error("failed to read stock mods directory: {0}")]
  ReadDir(#[from] io::Error),
}

async fn list_all_stock_mods(
  stock_mods_dir: &Path,
) -> Result<Vec<StockMod>, ListAllStockModsError> {
  let mut mods = Vec::new();
  let mut dir_entries = read_dir(stock_mods_dir).await?;

  while let Some(entry) = dir_entries.next_entry().await? {
    let path = entry.path();

    if !entry.file_type().await?.is_dir() {
      continue;
    }

    let modinfo_path = path.join("modinfo.json");

    match read_to_string(&modinfo_path).await {
      Ok(content) => match extract_stock_mod_from_modinfo(&content) {
        Ok(stock_mod) => {
          mods.push(stock_mod);
        }
        Err(_) => {
          continue;
        }
      },
      Err(_) => {
        continue;
      }
    }
  }

  Ok(mods)
}

#[derive(thiserror::Error, Debug)]
pub enum ListThirdPartyModsError {
  #[error("failed to read mods.json: {0}")]
  ReadModsJson(#[from] io::Error),

  #[error("failed to parse mods.json: {0}")]
  ParseModsJson(#[from] serde_json::Error),
}

pub async fn list_all_third_party_mods(
  game_variant: &GameVariant,
  resource_dir: &Path,
) -> Result<Vec<ThirdPartyMod>, ListThirdPartyModsError> {
  let mods_json_path = get_mods_resource_path(resource_dir);

  let content = match read_to_string(&mods_json_path).await {
    Ok(content) => content,
    Err(e) => return Err(ListThirdPartyModsError::ReadModsJson(e)),
  };

  let mods_data: HashMap<
    GameVariant,
    HashMap<String, serde_json::Value>,
  > = serde_json::from_str(&content)?;

  let variant_mods = match mods_data.get(game_variant) {
    Some(mods) => mods,
    None => return Ok(Vec::new()),
  };

  let mut mods = Vec::new();
  for mod_data in variant_mods.values() {
    let third_party_mod =
      serde_json::from_value::<ThirdPartyMod>(mod_data.clone());
    match third_party_mod {
      Ok(third_party_mod) => mods.push(third_party_mod),
      Err(e) => {
        return Err(ListThirdPartyModsError::ParseModsJson(e));
      }
    }
  }

  Ok(mods)
}

pub async fn list_cached_third_party_mods(
  game_variant: &GameVariant,
  mods_repository: &impl ModsRepository,
) -> Result<Vec<ThirdPartyMod>, ListCachedThirdPartyModsError> {
  mods_repository.get_third_party_mods(game_variant).await
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
  use async_trait::async_trait;
  use std::sync::{Arc, Mutex};
  use tempfile::tempdir;

  use crate::active_release::repository::sqlite_active_release_repository::SqliteActiveReleaseRepository;
  use crate::active_release::repository::ActiveReleaseRepository;
  use crate::filesystem::paths::get_or_create_asset_installation_dir;
  use crate::infra::http_client::ReqwestHttpClient;
  use crate::infra::testing::test_database::TestDatabase;
  use crate::mods::repository::sqlite_mods_repository::SqliteModsRepository;
  use crate::mods::types::ModInstallation;

  type TestResult<T = ()> =
    std::result::Result<T, Box<dyn std::error::Error>>;

  struct MockOnlineModRepo {
    mods: Vec<ThirdPartyMod>,
  }

  #[async_trait]
  impl OnlineModRepository for MockOnlineModRepo {
    async fn get_mods_for_variant(
      &self,
      _variant: &GameVariant,
      _client: &dyn HttpClient,
    ) -> Result<Vec<ThirdPartyMod>, FetchOnlineModsError> {
      Ok(self.mods.clone())
    }
  }

  fn create_sample_third_party_mod(id: &str) -> ThirdPartyMod {
    ThirdPartyMod {
      id: id.to_string(),
      name: format!("Name for {id}"),
      description: format!("Description for {id}"),
      category: "Content".to_string(),
      installation: ModInstallation {
        download_url: format!("https://example.com/{id}.zip"),
        modinfo: "modinfo.json".to_string(),
      },
      activity: None,
    }
  }

  #[test]
  fn test_extract_stock_mod_from_modinfo_valid() {
    let json = r#"[
      {
        "type": "MOD_INFO",
        "id": "dda_stock",
        "name": "DDA Stock",
        "description": "Stock mod for testing",
        "category": "content"
      }
    ]"#;

    let stock_mod = extract_stock_mod_from_modinfo(json).unwrap();
    assert_eq!(stock_mod.id, "dda_stock");
    assert_eq!(stock_mod.name, "DDA Stock");
    assert_eq!(stock_mod.description, "Stock mod for testing");
    assert_eq!(stock_mod.category, "content");
  }

  #[test]
  fn test_extract_stock_mod_from_modinfo_missing_optional_fields() {
    let json = r#"[
      {
        "type": "MOD_INFO",
        "id": "minimal_mod",
        "name": "Minimal Mod"
      }
    ]"#;

    let stock_mod = extract_stock_mod_from_modinfo(json).unwrap();
    assert_eq!(stock_mod.id, "minimal_mod");
    assert_eq!(stock_mod.name, "Minimal Mod");
    assert_eq!(stock_mod.description, "");
    assert_eq!(stock_mod.category, "");
  }

  #[test]
  fn test_extract_stock_mod_from_modinfo_errors() {
    // Missing MOD_INFO type
    let no_mod_info = r#"[{"type": "item", "id": "rock"}]"#;
    assert!(matches!(
      extract_stock_mod_from_modinfo(no_mod_info),
      Err(ExtractStockModError::NoModInfo)
    ));

    // Missing id
    let no_id = r#"[{"type": "MOD_INFO", "name": "No ID"}]"#;
    assert!(matches!(
      extract_stock_mod_from_modinfo(no_id),
      Err(ExtractStockModError::MissingField("id"))
    ));

    // Missing name
    let no_name = r#"[{"type": "MOD_INFO", "id": "no_name"}]"#;
    assert!(matches!(
      extract_stock_mod_from_modinfo(no_name),
      Err(ExtractStockModError::MissingField("name"))
    ));

    // Invalid JSON
    assert!(matches!(
      extract_stock_mod_from_modinfo("not json"),
      Err(ExtractStockModError::ParseModInfo(_))
    ));
  }

  #[tokio::test]
  async fn test_list_all_stock_mods_dir() -> TestResult {
    let temp = tempdir()?;
    let stock_dir = temp.path();

    // Valid mod 1
    let mod1_dir = stock_dir.join("mod1");
    tokio::fs::create_dir_all(&mod1_dir).await?;
    tokio::fs::write(
      mod1_dir.join("modinfo.json"),
      br#"[{"type": "MOD_INFO", "id": "mod1", "name": "Mod 1"}]"#,
    )
    .await?;

    // Valid mod 2
    let mod2_dir = stock_dir.join("mod2");
    tokio::fs::create_dir_all(&mod2_dir).await?;
    tokio::fs::write(
      mod2_dir.join("modinfo.json"),
      br#"[{"type": "MOD_INFO", "id": "mod2", "name": "Mod 2"}]"#,
    )
    .await?;

    // Non-directory file (should be ignored)
    tokio::fs::write(stock_dir.join("readme.txt"), b"hello").await?;

    // Directory without modinfo.json (should be ignored)
    let invalid_dir = stock_dir.join("invalid");
    tokio::fs::create_dir_all(&invalid_dir).await?;

    let stock_mods = list_all_stock_mods(stock_dir).await?;
    assert_eq!(stock_mods.len(), 2);
    assert!(stock_mods.iter().any(|m| m.id == "mod1"));
    assert!(stock_mods.iter().any(|m| m.id == "mod2"));

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_third_party_mods_resource() -> TestResult {
    let temp = tempdir()?;
    let content_dir = temp.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;

    let mods_json = r#"{
      "DarkDaysAhead": {
        "mod_a": {
          "id": "mod_a",
          "name": "Mod A",
          "description": "Desc A",
          "category": "content",
          "installation": {
            "download_url": "https://example.com/a.zip",
            "modinfo": "modinfo.json"
          },
          "activity": null
        }
      },
      "BrightNights": {
        "mod_b": {
          "id": "mod_b",
          "name": "Mod B",
          "description": "Desc B",
          "category": "content",
          "installation": {
            "download_url": "https://example.com/b.zip",
            "modinfo": "modinfo.json"
          },
          "activity": null
        }
      }
    }"#;
    tokio::fs::write(content_dir.join("mods.json"), mods_json)
      .await?;

    let dda_mods = list_all_third_party_mods(
      &GameVariant::DarkDaysAhead,
      temp.path(),
    )
    .await?;
    assert_eq!(dda_mods.len(), 1);
    assert_eq!(dda_mods[0].id, "mod_a");

    let bn_mods = list_all_third_party_mods(
      &GameVariant::BrightNights,
      temp.path(),
    )
    .await?;
    assert_eq!(bn_mods.len(), 1);
    assert_eq!(bn_mods[0].id, "mod_b");

    let tlg_mods = list_all_third_party_mods(
      &GameVariant::TheLastGeneration,
      temp.path(),
    )
    .await?;
    assert_eq!(tlg_mods.len(), 0);

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_third_party_mods_missing_file() -> TestResult
  {
    let temp = tempdir()?;
    let result = list_all_third_party_mods(
      &GameVariant::DarkDaysAhead,
      temp.path(),
    )
    .await;
    assert!(matches!(
      result,
      Err(ListThirdPartyModsError::ReadModsJson(_))
    ));

    Ok(())
  }

  #[tokio::test]
  async fn test_list_cached_third_party_mods_repo() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let repo = SqliteModsRepository::new(db.pool().clone());
    let variant = GameVariant::DarkDaysAhead;

    let sample = create_sample_third_party_mod("cached_1");
    repo.save_third_party_mods(&variant, vec![sample]).await?;

    let cached =
      list_cached_third_party_mods(&variant, &repo).await?;
    assert_eq!(cached.len(), 1);
    assert_eq!(cached[0].id, "cached_1");

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_mods_full_workflow() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_release_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let mods_repo = SqliteModsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;

    // 1. Setup cached mod in db
    let cached_mod = create_sample_third_party_mod("cached_mod");
    mods_repo
      .save_third_party_mods(&variant, vec![cached_mod])
      .await?;

    // 2. Setup online mod repo
    let online_mod = create_sample_third_party_mod("online_mod");
    let online_repos: Vec<Box<dyn OnlineModRepository>> =
      vec![Box::new(MockOnlineModRepo {
        mods: vec![online_mod],
      })];

    // 3. Setup bundled mods.json in resource_dir
    let content_dir = temp_res.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    let bundled_json = r#"{
      "DarkDaysAhead": {
        "bundled_mod": {
          "id": "bundled_mod",
          "name": "Bundled Mod",
          "description": "Bundled Description",
          "category": "content",
          "installation": {
            "download_url": "https://example.com/bundled.zip",
            "modinfo": "modinfo.json"
          },
          "activity": null
        }
      }
    }"#;
    tokio::fs::write(content_dir.join("mods.json"), bundled_json)
      .await?;

    // 4. Setup active release & stock mod
    let version = "v0.H";
    active_release_repo
      .set_active_release(&variant, version)
      .await?;

    let install_dir = get_or_create_asset_installation_dir(
      &variant,
      version,
      temp_data.path(),
    )
    .await?;
    let game_dir = install_dir.join("cataclysm-dda-v0.H");
    let stock_mods_dir = game_dir.join("data").join("mods");
    let stock_mod_dir = stock_mods_dir.join("stock_mod");
    tokio::fs::create_dir_all(&stock_mod_dir).await?;
    tokio::fs::write(
      stock_mod_dir.join("modinfo.json"),
      br#"[{"type": "MOD_INFO", "id": "stock_mod", "name": "Stock Mod"}]"#,
    )
    .await?;

    let updates: Arc<Mutex<Vec<ModsUpdatePayload>>> =
      Arc::new(Mutex::new(Vec::new()));
    let updates_clone = Arc::clone(&updates);

    let client = ReqwestHttpClient::new(reqwest::Client::new());

    list_all_mods(
      &variant,
      temp_data.path(),
      temp_res.path(),
      &os,
      &active_release_repo,
      &mods_repo,
      &online_repos,
      &client,
      move |payload| {
        updates_clone.lock().unwrap().push(payload);
        Ok::<(), std::io::Error>(())
      },
    )
    .await?;

    let recorded_updates = updates.lock().unwrap().clone();
    assert!(recorded_updates.len() >= 2);

    // Initial update has Fetching status
    assert_eq!(
      recorded_updates[0].status,
      ModsUpdateStatus::Fetching
    );

    // Final update has Success status and contains all mods
    let final_update = recorded_updates.last().unwrap();
    assert_eq!(final_update.status, ModsUpdateStatus::Success);

    let mod_ids: Vec<String> = final_update
      .mods
      .iter()
      .map(|m| match m {
        Mod::ThirdParty(tp) => tp.id.clone(),
        Mod::Stock(s) => s.id.clone(),
      })
      .collect();

    assert!(mod_ids.contains(&"cached_mod".to_string()));
    assert!(mod_ids.contains(&"online_mod".to_string()));
    assert!(mod_ids.contains(&"bundled_mod".to_string()));
    assert!(mod_ids.contains(&"stock_mod".to_string()));

    // Verify all third-party mods were saved in database
    let db_cached = mods_repo.get_third_party_mods(&variant).await?;
    let db_ids: Vec<String> =
      db_cached.into_iter().map(|m| m.id).collect();
    assert!(db_ids.contains(&"cached_mod".to_string()));
    assert!(db_ids.contains(&"online_mod".to_string()));
    assert!(db_ids.contains(&"bundled_mod".to_string()));

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_mods_no_active_release() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_release_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let mods_repo = SqliteModsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;

    // Resource dir with empty mods.json
    let content_dir = temp_res.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    tokio::fs::write(content_dir.join("mods.json"), "{}").await?;

    let final_payload: Arc<Mutex<Option<ModsUpdatePayload>>> =
      Arc::new(Mutex::new(None));
    let final_payload_clone = Arc::clone(&final_payload);

    let client = ReqwestHttpClient::new(reqwest::Client::new());

    list_all_mods(
      &variant,
      temp_data.path(),
      temp_res.path(),
      &os,
      &active_release_repo,
      &mods_repo,
      &[],
      &client,
      move |payload| {
        *final_payload_clone.lock().unwrap() = Some(payload);
        Ok::<(), std::io::Error>(())
      },
    )
    .await?;

    let result = final_payload.lock().unwrap().take().unwrap();
    assert_eq!(result.status, ModsUpdateStatus::Success);
    // No stock mods because active release was None
    assert!(result.mods.is_empty());

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_mods_deduplication() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_release_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let mods_repo = SqliteModsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;

    // Same mod in cache and online and bundled
    let duplicate_mod = create_sample_third_party_mod("duplicate_id");
    mods_repo
      .save_third_party_mods(&variant, vec![duplicate_mod.clone()])
      .await?;

    let online_repos: Vec<Box<dyn OnlineModRepository>> =
      vec![Box::new(MockOnlineModRepo {
        mods: vec![duplicate_mod],
      })];

    let content_dir = temp_res.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    let bundled_json = r#"{
      "DarkDaysAhead": {
        "duplicate_id": {
          "id": "duplicate_id",
          "name": "Duplicate Mod",
          "description": "Desc",
          "category": "content",
          "installation": {
            "download_url": "https://example.com/dup.zip",
            "modinfo": "modinfo.json"
          },
          "activity": null
        }
      }
    }"#;
    tokio::fs::write(content_dir.join("mods.json"), bundled_json)
      .await?;

    let final_payload: Arc<Mutex<Option<ModsUpdatePayload>>> =
      Arc::new(Mutex::new(None));
    let final_payload_clone = Arc::clone(&final_payload);
    let client = ReqwestHttpClient::new(reqwest::Client::new());

    list_all_mods(
      &variant,
      temp_data.path(),
      temp_res.path(),
      &os,
      &active_release_repo,
      &mods_repo,
      &online_repos,
      &client,
      move |payload| {
        *final_payload_clone.lock().unwrap() = Some(payload);
        Ok::<(), std::io::Error>(())
      },
    )
    .await?;

    let result = final_payload.lock().unwrap().take().unwrap();
    let dup_count = result
      .mods
      .iter()
      .filter(|m| match m {
        Mod::ThirdParty(tp) => tp.id == "duplicate_id",
        Mod::Stock(s) => s.id == "duplicate_id",
      })
      .count();
    assert_eq!(dup_count, 1);

    Ok(())
  }

  #[tokio::test]
  async fn test_list_all_mods_on_update_error() -> TestResult {
    let db = TestDatabase::builder().build()?;
    let active_release_repo =
      SqliteActiveReleaseRepository::new(db.pool().clone());
    let mods_repo = SqliteModsRepository::new(db.pool().clone());
    let temp_data = tempdir()?;
    let temp_res = tempdir()?;
    let os = OS::Linux;
    let variant = GameVariant::DarkDaysAhead;

    let content_dir = temp_res.path().join("content");
    tokio::fs::create_dir_all(&content_dir).await?;
    tokio::fs::write(content_dir.join("mods.json"), "{}").await?;

    let client = ReqwestHttpClient::new(reqwest::Client::new());

    let result = list_all_mods(
      &variant,
      temp_data.path(),
      temp_res.path(),
      &os,
      &active_release_repo,
      &mods_repo,
      &[],
      &client,
      |_payload| {
        Err(std::io::Error::new(
          std::io::ErrorKind::BrokenPipe,
          "channel closed",
        ))
      },
    )
    .await;

    assert!(matches!(result, Err(ListAllModsError::Send(_))));

    Ok(())
  }
}
