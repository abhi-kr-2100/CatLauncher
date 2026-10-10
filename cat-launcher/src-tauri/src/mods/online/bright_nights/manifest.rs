use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::mods::{
  get_last_activity_for_third_party_mod::extract_repo_from_github_url,
  types::{ModActivity, ModInstallation, ThirdPartyMod},
};

#[derive(Debug, Error)]
pub enum IntoThirdPartyModError {
  #[error("extract path is required")]
  MissingExtractPath,

  #[error("unsupported source type: {0}")]
  UnsupportedSourceType(String),

  #[error("failed to extract repo from github url: {0}")]
  FailedToExtractRepoFromGithubUrl(String),

  #[error("failed to extract archive filename from url: {0}")]
  FailedToExtractArchiveFilename(String),

  #[error("failed to extract branch from archive filename: {0}")]
  FailedToExtractBranch(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BrightNightsOnlineModSource {
  #[serde(rename = "type")]
  pub source_type: String,
  pub url: String,
  pub extract_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BrightNightsOnlineMod {
  pub id: String,
  pub display_name: String,
  pub description: Option<String>,
  pub short_description: Option<String>,
  pub categories: Option<Vec<String>>,
  pub source: BrightNightsOnlineModSource,
}

impl BrightNightsOnlineMod {
  pub fn into_third_party_mod(
    self,
  ) -> Result<ThirdPartyMod, IntoThirdPartyModError> {
    self.validate_source_type()?;
    let (repo, repo_name) = self.extract_repo_and_name()?;
    let branch = self.extract_branch_from_url()?;
    let directory_name = format!("{}-{}", repo_name, branch);
    let modinfo_path = self.get_modinfo_path(&directory_name)?;
    let mod_inst = self.build_mod_installation(modinfo_path);
    let activity = self.build_activity(&repo);
    let category = self.get_category();

    Ok(ThirdPartyMod {
      id: self.id,
      name: self.display_name,
      description: self
        .description
        .or(self.short_description)
        .unwrap_or_default(),
      category,
      installation: mod_inst,
      activity: Some(activity),
    })
  }

  fn validate_source_type(
    &self,
  ) -> Result<(), IntoThirdPartyModError> {
    if self.source.source_type != "github_archive" {
      return Err(IntoThirdPartyModError::UnsupportedSourceType(
        self.source.source_type.clone(),
      ));
    }
    Ok(())
  }

  fn extract_repo_and_name(
    &self,
  ) -> Result<(String, String), IntoThirdPartyModError> {
    let repo = match extract_repo_from_github_url(&self.source.url) {
      Some(repo) => repo,
      None => {
        return Err(
          IntoThirdPartyModError::FailedToExtractRepoFromGithubUrl(
            self.source.url.clone(),
          ),
        );
      }
    };

    let repo_name = match repo.split_once("/") {
      Some((_, repo)) => repo.to_string(),
      None => {
        return Err(
          IntoThirdPartyModError::FailedToExtractRepoFromGithubUrl(
            self.source.url.clone(),
          ),
        );
      }
    };

    Ok((repo, repo_name))
  }

  fn extract_branch_from_url(
    &self,
  ) -> Result<String, IntoThirdPartyModError> {
    let archive_filename = match self.source.url.rsplit_once("/") {
      Some((_, filename)) => filename.to_string(),
      None => {
        return Err(
          IntoThirdPartyModError::FailedToExtractArchiveFilename(
            self.source.url.clone(),
          ),
        );
      }
    };

    match Path::new(&archive_filename).file_stem() {
      Some(branch) => Ok(branch.to_string_lossy().to_string()),
      None => Err(IntoThirdPartyModError::FailedToExtractBranch(
        self.source.url.clone(),
      )),
    }
  }

  fn get_modinfo_path(
    &self,
    directory_name: &str,
  ) -> Result<String, IntoThirdPartyModError> {
    let path = match &self.source.extract_path {
      Some(path) => {
        Path::new(directory_name).join(path).join("modinfo.json")
      }
      None => return Err(IntoThirdPartyModError::MissingExtractPath),
    };

    Ok(path.to_string_lossy().to_string())
  }

  fn build_mod_installation(
    &self,
    modinfo_path: String,
  ) -> ModInstallation {
    ModInstallation {
      download_url: self.source.url.clone(),
      modinfo: modinfo_path,
    }
  }

  fn build_activity(&self, repo: &str) -> ModActivity {
    ModActivity::GithubCommit {
      github: format!("https://github.com/{}", repo),
    }
  }

  fn get_category(&self) -> String {
    self
      .categories
      .clone()
      .and_then(|c| c.into_iter().next())
      .unwrap_or_else(|| "Unknown".to_string())
  }
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

  fn create_sample_mod(
    source_type: &str,
    url: &str,
    extract_path: Option<&str>,
  ) -> BrightNightsOnlineMod {
    BrightNightsOnlineMod {
      id: "bn_sample_mod".to_string(),
      display_name: "BN Sample Mod".to_string(),
      description: Some("Detailed description".to_string()),
      short_description: Some("Short description".to_string()),
      categories: Some(vec![
        "content".to_string(),
        "items".to_string(),
      ]),
      source: BrightNightsOnlineModSource {
        source_type: source_type.to_string(),
        url: url.to_string(),
        extract_path: extract_path.map(|s| s.to_string()),
      },
    }
  }

  #[test]
  fn test_into_third_party_mod_success() {
    let raw_mod = create_sample_mod(
      "github_archive",
      "https://github.com/cataclysmbnteam/Cataclysm-BN/archive/refs/heads/master.zip",
      Some("mods/sample"),
    );

    let tp = raw_mod.into_third_party_mod().unwrap();
    assert_eq!(tp.id, "bn_sample_mod");
    assert_eq!(tp.name, "BN Sample Mod");
    assert_eq!(tp.description, "Detailed description");
    assert_eq!(tp.category, "content");
    assert_eq!(
      tp.installation.download_url,
      "https://github.com/cataclysmbnteam/Cataclysm-BN/archive/refs/heads/master.zip"
    );
    let expected_modinfo = Path::new("Cataclysm-BN-master")
      .join("mods")
      .join("sample")
      .join("modinfo.json");
    assert_eq!(Path::new(&tp.installation.modinfo), expected_modinfo);

    match tp.activity {
      Some(ModActivity::GithubCommit { github }) => {
        assert_eq!(
          github,
          "https://github.com/cataclysmbnteam/Cataclysm-BN"
        );
      }
      None => panic!("Expected github activity"),
    }
  }

  #[test]
  fn test_into_third_party_mod_fallback_to_short_description() {
    let mut raw_mod = create_sample_mod(
      "github_archive",
      "https://github.com/cataclysmbnteam/Cataclysm-BN/archive/refs/heads/main.zip",
      Some("mods/sample"),
    );
    raw_mod.description = None;

    let tp = raw_mod.into_third_party_mod().unwrap();
    assert_eq!(tp.description, "Short description");
  }

  #[test]
  fn test_into_third_party_mod_no_description() {
    let mut raw_mod = create_sample_mod(
      "github_archive",
      "https://github.com/cataclysmbnteam/Cataclysm-BN/archive/refs/heads/main.zip",
      Some("mods/sample"),
    );
    raw_mod.description = None;
    raw_mod.short_description = None;

    let tp = raw_mod.into_third_party_mod().unwrap();
    assert_eq!(tp.description, "");
  }

  #[test]
  fn test_into_third_party_mod_default_category() {
    let mut raw_mod = create_sample_mod(
      "github_archive",
      "https://github.com/cataclysmbnteam/Cataclysm-BN/archive/refs/heads/main.zip",
      Some("mods/sample"),
    );
    raw_mod.categories = None;
    let tp = raw_mod.clone().into_third_party_mod().unwrap();
    assert_eq!(tp.category, "Unknown");

    raw_mod.categories = Some(vec![]);
    let tp2 = raw_mod.into_third_party_mod().unwrap();
    assert_eq!(tp2.category, "Unknown");
  }

  #[test]
  fn test_into_third_party_mod_unsupported_source_type() {
    let raw_mod = create_sample_mod(
      "git",
      "https://github.com/cataclysmbnteam/Cataclysm-BN/archive/refs/heads/main.zip",
      Some("mods/sample"),
    );

    let err = raw_mod.into_third_party_mod().unwrap_err();
    assert!(matches!(
      err,
      IntoThirdPartyModError::UnsupportedSourceType(t) if t == "git"
    ));
  }

  #[test]
  fn test_into_third_party_mod_missing_extract_path() {
    let raw_mod = create_sample_mod(
      "github_archive",
      "https://github.com/cataclysmbnteam/Cataclysm-BN/archive/refs/heads/main.zip",
      None,
    );

    let err = raw_mod.into_third_party_mod().unwrap_err();
    assert!(matches!(
      err,
      IntoThirdPartyModError::MissingExtractPath
    ));
  }

  #[test]
  fn test_into_third_party_mod_invalid_github_url() {
    let raw_mod = create_sample_mod(
      "github_archive",
      "https://github.com/only-owner",
      Some("mods/sample"),
    );

    let err = raw_mod.into_third_party_mod().unwrap_err();
    assert!(matches!(
      err,
      IntoThirdPartyModError::FailedToExtractRepoFromGithubUrl(_)
    ));
  }

  #[test]
  fn test_deserialize_and_convert_json() {
    let json = r#"{
      "id": "magiclysm_bn",
      "display_name": "Magiclysm BN",
      "description": "Magic fantasy in Bright Nights",
      "categories": ["content", "magic"],
      "source": {
        "type": "github_archive",
        "url": "https://github.com/cataclysmbnteam/magiclysm/archive/refs/heads/main.zip",
        "extract_path": "Magiclysm"
      }
    }"#;

    let online_mod: BrightNightsOnlineMod =
      serde_json::from_str(json).unwrap();
    let tp = online_mod.into_third_party_mod().unwrap();
    assert_eq!(tp.id, "magiclysm_bn");
    assert_eq!(tp.name, "Magiclysm BN");
    assert_eq!(tp.category, "content");
    let expected_modinfo = Path::new("magiclysm-main")
      .join("Magiclysm")
      .join("modinfo.json");
    assert_eq!(Path::new(&tp.installation.modinfo), expected_modinfo);
  }
}
