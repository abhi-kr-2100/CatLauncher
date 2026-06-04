use crate::util::hash;

use super::types::{Release, SimpleUser};

impl Release {
    pub fn new(owner: &str, repo: &str, tag_name: &str) -> Self {
        let id = hash(&format!("release:{owner}/{repo}:{tag_name}"));
        let base = format!("https://api.github.com/repos/{owner}/{repo}");
        let html_base = format!("https://github.com/{owner}/{repo}");
        let created_at = super::DEFAULT_TIMESTAMP.to_string();

        Self {
            owner: owner.to_string(),
            repo: repo.to_string(),
            url: format!("{base}/releases/{id}"),
            html_url: format!("{html_base}/releases/tag/{tag_name}"),
            assets_url: format!("{base}/releases/{id}/assets"),
            upload_url: format!("{base}/releases/{id}/assets{{?name,label}}"),
            tarball_url: Some(format!("{base}/tarball/{tag_name}")),
            zipball_url: Some(format!("{base}/zipball/{tag_name}")),
            id,
            node_id: format!("mock_node_id_{id}"),
            tag_name: tag_name.to_string(),
            target_commitish: "main".to_string(),
            name: None,
            body: None,
            draft: false,
            prerelease: false,
            immutable: None,
            created_at: created_at.clone(),
            published_at: Some(created_at.clone()),
            updated_at: None,
            author: SimpleUser::new(owner),
            assets: Vec::new(),
            body_html: None,
            body_text: None,
            mentions_count: None,
            discussion_url: None,
            reactions: None,
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn target_commitish(mut self, commitish: impl Into<String>) -> Self {
        self.target_commitish = commitish.into();
        self
    }

    pub fn draft(mut self, draft: bool) -> Self {
        self.draft = draft;
        if draft {
            self.published_at = None;
        } else {
            self.published_at = Some(self.created_at.clone())
        }
        self
    }

    pub fn prerelease(mut self, prerelease: bool) -> Self {
        self.prerelease = prerelease;
        self
    }

    pub fn created_at(mut self, created_at: impl Into<String>) -> Self {
        self.created_at = created_at.into();
        if !self.draft {
            self.published_at = Some(self.created_at.clone())
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Release;

    #[test]
    fn test_release_new_defaults() {
        let release = Release::new("octocat", "hello-world", "v1.0.0");
        assert_eq!(release.tag_name, "v1.0.0");
        assert_eq!(release.target_commitish, "main");
        assert!(!release.draft);
        assert!(!release.prerelease);
        assert!(release.name.is_none());
        assert!(release.body.is_none());
        assert!(release.published_at.is_some());
        assert_eq!(release.author.login, "octocat");
        assert!(release.assets.is_empty());
    }

    #[test]
    fn test_release_builder() {
        let release = Release::new("test-user", "my-repo", "v2.0.0")
            .name("v2.0.0")
            .body("A major release")
            .target_commitish("develop")
            .draft(false)
            .prerelease(true);

        assert_eq!(release.name, Some("v2.0.0".to_string()));
        assert_eq!(release.body, Some("A major release".to_string()));
        assert_eq!(release.target_commitish, "develop");
        assert!(!release.draft);
        assert!(release.prerelease);
        assert!(release.published_at.is_some());
    }

    #[test]
    fn test_release_draft_sets_published_at_null() {
        let release = Release::new("user", "repo", "v0.1.0").draft(true);
        assert!(release.draft);
        assert!(release.published_at.is_none());
    }

    #[test]
    fn test_release_has_hash_id() {
        let release = Release::new("user", "repo", "v1.0.0");
        assert_ne!(release.id, 0);
        assert!(release.node_id.starts_with("mock_node_id_"));
    }

    #[test]
    fn test_release_deterministic_id() {
        let a = Release::new("user", "repo", "v1.0.0");
        let b = Release::new("user", "repo", "v1.0.0");
        assert_eq!(a.id, b.id);
    }

    #[test]
    fn test_release_diff_tag_diff_id() {
        let a = Release::new("user", "repo", "v1.0.0");
        let b = Release::new("user", "repo", "v2.0.0");
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn test_release_urls() {
        let release = Release::new("octocat", "hello-world", "v1.0.0");
        assert_eq!(
            release.url,
            format!("https://api.github.com/repos/octocat/hello-world/releases/{}", release.id)
        );
        assert_eq!(
            release.html_url,
            "https://github.com/octocat/hello-world/releases/tag/v1.0.0"
        );
        assert!(release.tarball_url.is_some());
        assert!(release.zipball_url.is_some());
    }
}
