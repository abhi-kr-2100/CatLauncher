use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::util::{LoadError, load_json_from_file};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GitUser {
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitTree {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Verification {
    pub verified: bool,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitParent {
    pub sha: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SimpleUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub site_admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_view_type: Option<String>,
}

impl SimpleUser {
    pub(crate) fn new(login: &str) -> Self {
        let id = crate::util::hash(&format!("user:{login}"));
        Self {
            name: None,
            email: None,
            login: login.to_string(),
            id,
            node_id: format!("mock_node_id_{id}"),
            avatar_url: format!("https://avatars.githubusercontent.com/u/{id}?v=4"),
            gravatar_id: None,
            url: format!("https://api.github.com/users/{login}"),
            html_url: format!("https://github.com/{login}"),
            followers_url: format!("https://api.github.com/users/{login}/followers"),
            following_url: format!("https://api.github.com/users/{login}/following"),
            gists_url: format!("https://api.github.com/users/{login}/gists{{/gist_id}}"),
            starred_url: format!("https://api.github.com/users/{login}/starred{{/owner}}{{/repo}}"),
            subscriptions_url: format!("https://api.github.com/users/{login}/subscriptions"),
            organizations_url: format!("https://api.github.com/users/{login}/orgs"),
            repos_url: format!("https://api.github.com/users/{login}/repos"),
            events_url: format!("https://api.github.com/users/{login}/events{{/privacy}}"),
            received_events_url: format!("https://api.github.com/users/{login}/received_events"),
            user_type: "User".to_string(),
            site_admin: false,
            starred_at: None,
            user_view_type: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommitDetail {
    pub url: String,
    pub author: Option<GitUser>,
    pub committer: Option<GitUser>,
    pub message: String,
    pub comment_count: u64,
    pub tree: CommitTree,
    pub verification: Verification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Commit {
    pub sha: String,
    pub node_id: String,
    pub commit: CommitDetail,
    pub url: String,
    pub html_url: String,
    pub comments_url: String,
    pub author: Option<SimpleUser>,
    pub committer: Option<SimpleUser>,
    pub parents: Vec<CommitParent>,
    #[serde(skip)]
    pub(crate) owner: String,
    #[serde(skip)]
    pub(crate) repo: String,
}

impl Commit {
    /// Load commits from a JSON file.
    ///
    /// The JSON file should contain an array of commit objects as returned by the GitHub API.
    pub fn load_from_file(
        path: impl AsRef<Path>,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<Self>, LoadError> {
        let mut commits: Vec<Self> = load_json_from_file(path)?;

        for commit in &mut commits {
            commit.owner = owner.to_string();
            commit.repo = repo.to_string();
        }

        Ok(commits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_from_file_happy_path() -> Result<(), Box<dyn std::error::Error>> {
        let commits = Commit::load_from_file("testing/data/commits.json", "owner1", "repo1")?;

        assert_eq!(commits.len(), 30);

        assert_eq!(commits[0].sha, "9291e608e354242c8ff12d47896799d456719922");

        assert_eq!(commits[0].owner, "owner1");
        assert_eq!(commits[0].repo, "repo1");

        Ok(())
    }

    #[test]
    fn test_load_from_file_io_error() {
        let result = Commit::load_from_file("non_existent_file.json", "o", "r");
        assert!(matches!(result, Err(LoadError::Io { .. })));
    }

    #[test]
    fn test_load_from_file_json_error() {
        let result = Commit::load_from_file("testing/data/commits_invalid.json", "o", "r");
        assert!(matches!(result, Err(LoadError::Json { .. })));
    }
}
