use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::util::{LoadError, load_json_from_file};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RepositoryOwner {
    pub login: String,
    pub id: u64,
    pub avatar_url: String,
    pub gravatar_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub html_url: String,
    pub repos_url: String,
    #[serde(rename = "type")]
    pub owner_type: String,
}

impl RepositoryOwner {
    pub(crate) fn new(login: &str) -> Self {
        let id = crate::util::hash(&format!("owner:{login}"));
        Self {
            login: login.to_string(),
            id,
            avatar_url: format!("https://avatars.githubusercontent.com/u/{id}?v=4"),
            gravatar_id: String::new(),
            url: Some(format!("https://api.github.com/users/{login}")),
            html_url: format!("https://github.com/{login}"),
            repos_url: format!("https://api.github.com/users/{login}/repos"),
            owner_type: "User".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RepositoryLicense {
    pub key: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spdx_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Repository {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub owner: RepositoryOwner,
    pub private: bool,
    pub html_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub deployments_url: String,
    pub downloads_url: String,
    pub events_url: String,
    pub forks_url: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub pulls_url: String,
    pub releases_url: String,
    pub ssh_url: String,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub clone_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror_url: Option<String>,
    pub hooks_url: String,
    pub svn_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub forks_count: u64,
    pub stargazers_count: u64,
    pub watchers_count: u64,
    pub size: u64,
    pub default_branch: String,
    pub open_issues_count: u64,
    pub is_template: bool,
    pub topics: Vec<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub has_downloads: bool,
    pub has_discussions: bool,
    pub archived: bool,
    pub disabled: bool,
    pub visibility: String,
    pub pushed_at: String,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<RepositoryLicense>,
    pub allow_forking: bool,
    pub web_commit_signoff_required: bool,
}

impl Repository {
    /// Load repositories from a JSON file.
    ///
    /// The JSON file should contain an array of repository objects as returned by the GitHub API.
    pub fn load_from_file(path: impl AsRef<Path>) -> Result<Vec<Self>, LoadError> {
        load_json_from_file(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_file_happy_path() -> Result<(), Box<dyn std::error::Error>> {
        let repos = Repository::load_from_file("testing/data/repositories.json")?;

        assert_eq!(repos.len(), 30);
        assert_eq!(repos[0].name, "arxiv-sanity-lite");

        Ok(())
    }

    #[test]
    fn test_load_from_file_io_error() {
        let result = Repository::load_from_file("non_existent_file.json");
        assert!(matches!(result, Err(LoadError::Io { .. })));
    }

    #[test]
    fn test_load_from_file_json_error() {
        let result = Repository::load_from_file("testing/data/repositories_invalid.json");
        assert!(matches!(result, Err(LoadError::Json { .. })));
    }
}
