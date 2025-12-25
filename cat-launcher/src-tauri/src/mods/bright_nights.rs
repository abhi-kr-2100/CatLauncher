use reqwest::Client;
use serde::Deserialize;

use crate::mods::types::ThirdPartyMod;

const BRIGHT_NIGHTS_MOD_REPOSITORY_URL: &str =
  "https://mods.cataclysmbn.org/generated/mods.json";

#[derive(thiserror::Error, Debug)]
pub enum FetchBrightNightsModsError {
  #[error("failed to fetch mods from mod repository: {0}")]
  Fetch(#[from] reqwest::Error),

  #[error("failed to parse mods from mod repository: {0}")]
  Parse(#[from] serde_json::Error),

  #[error("unsupported mod source url: {0}")]
  UnsupportedSourceUrl(String),
}

#[derive(Debug, Deserialize)]
struct OnlineModEntry {
  pub id: String,
  pub display_name: String,
  pub short_description: String,
  #[serde(default)]
  pub categories: Vec<String>,
  #[serde(default)]
  pub homepage: Option<String>,
  pub source: OnlineModSource,
}

#[derive(Debug, Deserialize)]
struct OnlineModSource {
  #[serde(rename = "type")]
  pub source_type: String,
  pub url: String,
  pub extract_path: String,
}

fn github_archive_root_dir_name(url: &str) -> Option<String> {
  let parsed_url = url::Url::parse(url).ok()?;
  let segments: Vec<&str> = parsed_url.path_segments()?.collect();

  if segments.len() < 3 {
    return None;
  }

  let repo = segments.get(1)?;
  let last = segments.last()?;
  let suffix = last.strip_suffix(".zip")?;

  Some(format!("{}-{}", repo, suffix))
}

fn build_modinfo_relative_path(
  source_url: &str,
  extract_path: &str,
) -> Result<String, FetchBrightNightsModsError> {
  let root_dir_name = github_archive_root_dir_name(source_url)
    .ok_or_else(|| {
      FetchBrightNightsModsError::UnsupportedSourceUrl(
        source_url.to_string(),
      )
    })?;

  let extract_path = extract_path.trim_matches('/');

  let mod_dir = if extract_path.starts_with(&root_dir_name) {
    extract_path.to_string()
  } else {
    format!("{}/{}", root_dir_name, extract_path)
  };

  Ok(format!("{}/modinfo.json", mod_dir))
}

fn to_third_party_mod(
  mod_entry: OnlineModEntry,
) -> Result<ThirdPartyMod, FetchBrightNightsModsError> {
  if mod_entry.source.source_type != "github_archive" {
    return Err(FetchBrightNightsModsError::UnsupportedSourceUrl(
      mod_entry.source.url,
    ));
  }

  let category =
    mod_entry.categories.first().cloned().unwrap_or_default();

  let github = mod_entry
    .homepage
    .filter(|h| !h.is_empty())
    .unwrap_or_else(|| mod_entry.source.url.clone());

  let modinfo = build_modinfo_relative_path(
    &mod_entry.source.url,
    &mod_entry.source.extract_path,
  )?;

  Ok(ThirdPartyMod {
    id: mod_entry.id,
    name: mod_entry.display_name,
    description: mod_entry.short_description,
    category,
    installation: crate::mods::types::ModInstallation {
      download_url: mod_entry.source.url,
      modinfo,
    },
    activity: crate::mods::types::ModActivity {
      activity_type: "github_commit".to_string(),
      github,
    },
  })
}

pub async fn fetch_bright_nights_mods(
  client: &Client,
) -> Result<Vec<ThirdPartyMod>, FetchBrightNightsModsError> {
  let text = client
    .get(BRIGHT_NIGHTS_MOD_REPOSITORY_URL)
    .send()
    .await?
    .error_for_status()?
    .text()
    .await?;

  let repo_mods: Vec<OnlineModEntry> = serde_json::from_str(&text)?;

  let mut mods = Vec::new();
  for repo_mod in repo_mods {
    let id = repo_mod.id.clone();
    let source_url = repo_mod.source.url.clone();

    match to_third_party_mod(repo_mod) {
      Ok(third_party_mod) => mods.push(third_party_mod),
      Err(e) => {
        eprintln!(
          "Failed to parse online mod {id} (source: {source_url}): {e}",
        );
      }
    }
  }

  Ok(mods)
}
