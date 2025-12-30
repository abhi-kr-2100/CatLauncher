use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::mods::types::ThirdPartyMod;

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteMod {
    pub id: String,
    #[serde(rename = "display_name")]
    pub name: String,
    #[serde(rename = "short_description")]
    pub description: String,
    pub categories: Vec<String>,
    pub source: Source,
    pub homepage: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub url: String,
    #[serde(rename = "extract_path")]
    pub extract_path: String,
}

use once_cell::sync::Lazy;
use regex::Regex;

fn get_repo_name_and_branch_from_url(url: &str) -> (Option<String>, Option<String>) {
    static RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"github\.com/[^/]+/([^/]+)/archive/refs/heads/([^.]+)\.zip").unwrap()
    });
    let caps = RE.captures(url);
    if let Some(caps) = caps {
        (caps.get(1).map(|m| m.as_str().to_string()), caps.get(2).map(|m| m.as_str().to_string()))
    } else {
        (None, None)
    }
}

pub fn translate_mods(remote_mods: Vec<RemoteMod>) -> HashMap<String, ThirdPartyMod> {
    let mut translated_mods = HashMap::new();

    for remote_mod in remote_mods {
        let (repo_name, branch_name) = get_repo_name_and_branch_from_url(&remote_mod.source.url);

        let modinfo = if let (Some(repo), Some(branch)) = (repo_name, branch_name) {
            format!("{}-{}/{}/modinfo.json", repo, branch, remote_mod.source.extract_path)
        } else {
            String::new()
        };

        let github_repo_url = remote_mod.homepage
            .split("/tree/")
            .next()
            .unwrap_or(&remote_mod.homepage)
            .to_string();

        let translated_mod = ThirdPartyMod {
            id: remote_mod.id.clone(),
            name: remote_mod.name,
            description: remote_mod.description,
            category: remote_mod.categories.get(0).cloned().unwrap_or_default(),
            installation: crate::mods::types::ModInstallation {
                download_url: remote_mod.source.url,
                modinfo,
            },
            activity: crate::mods::types::ModActivity {
                activity_type: "github_commit".to_string(),
                github: github_repo_url,
            },
        };
        translated_mods.insert(remote_mod.id, translated_mod);
    }

    translated_mods
}
