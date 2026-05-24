use serde::{Deserialize, Serialize};

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
pub struct CommitStats {
    pub additions: u64,
    pub deletions: u64,
    pub total: u64,
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
pub struct DiffEntry {
    pub sha: Option<String>,
    pub filename: String,
    pub status: String,
    pub additions: u64,
    pub deletions: u64,
    pub changes: u64,
    pub blob_url: String,
    pub raw_url: String,
    pub contents_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_filename: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Commit {
    pub url: String,
    pub sha: String,
    pub node_id: String,
    pub html_url: String,
    pub comments_url: String,
    pub commit: CommitDetail,
    pub author: Option<SimpleUser>,
    pub committer: Option<SimpleUser>,
    pub parents: Vec<CommitParent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<CommitStats>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<DiffEntry>,
    #[serde(skip)]
    pub(crate) owner: String,
    #[serde(skip)]
    pub(crate) repo: String,
}
