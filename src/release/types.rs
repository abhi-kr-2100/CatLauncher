use serde::{Deserialize, Serialize};

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
    #[serde(default)]
    pub gravatar_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
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
            name: Some(login.to_string()),
            email: None,
            login: login.to_string(),
            id,
            node_id: format!("mock_node_id_{id}"),
            avatar_url: format!("https://avatars.githubusercontent.com/u/{id}?v=4"),
            gravatar_id: String::new(),
            url: Some(format!("https://api.github.com/users/{login}")),
            html_url: format!("https://github.com/{login}"),
            followers_url: format!("https://api.github.com/users/{login}/followers"),
            following_url: format!(
                "https://api.github.com/users/{login}/following{{/other_user}}"
            ),
            gists_url: format!("https://api.github.com/users/{login}/gists{{/gist_id}}"),
            starred_url: format!(
                "https://api.github.com/users/{login}/starred{{/owner}}{{/repo}}"
            ),
            subscriptions_url: format!("https://api.github.com/users/{login}/subscriptions"),
            organizations_url: format!("https://api.github.com/users/{login}/orgs"),
            repos_url: format!("https://api.github.com/users/{login}/repos"),
            events_url: format!("https://api.github.com/users/{login}/events{{/privacy}}"),
            received_events_url: format!(
                "https://api.github.com/users/{login}/received_events"
            ),
            user_type: "User".to_string(),
            site_admin: false,
            starred_at: None,
            user_view_type: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReleaseAsset {
    pub url: String,
    pub browser_download_url: String,
    pub id: u64,
    pub node_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub state: String,
    pub content_type: String,
    pub size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    pub download_count: u64,
    pub created_at: String,
    pub updated_at: String,
    pub uploader: Option<SimpleUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ReactionRollup {
    pub url: String,
    pub total_count: u64,
    #[serde(rename = "+1")]
    pub plus_one: u64,
    #[serde(rename = "-1")]
    pub minus_one: u64,
    pub laugh: u64,
    pub confused: u64,
    pub heart: u64,
    pub hooray: u64,
    pub eyes: u64,
    pub rocket: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Release {
    #[serde(skip)]
    pub(crate) owner_login: String,
    #[serde(skip)]
    pub(crate) repo_name: String,
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub tarball_url: Option<String>,
    pub zipball_url: Option<String>,
    pub id: u64,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub draft: bool,
    pub prerelease: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immutable: Option<bool>,
    pub created_at: String,
    pub published_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    pub author: SimpleUser,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<ReleaseAsset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discussion_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reactions: Option<ReactionRollup>,
}
