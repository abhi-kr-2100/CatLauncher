use super::types::{Repository, RepositoryOwner};

impl Repository {
    pub fn new(owner: &str, name: &str) -> Self {
        let id = crate::util::hash(&format!("repository:{owner}/{name}"));
        let full_name = format!("{owner}/{name}");
        let base = format!("https://api.github.com/repos/{owner}/{name}");
        let html_base = format!("https://github.com/{owner}/{name}");

        Self {
            id,
            node_id: format!("mock_node_id_{id}"),
            name: name.to_string(),
            full_name,
            owner: RepositoryOwner::new(owner),
            private: false,
            html_url: html_base.clone(),
            description: None,
            fork: false,
            url: base.clone(),
            archive_url: format!("{base}/{{archive_format}}{{/ref}}"),
            assignees_url: format!("{base}/assignees{{/assignee}}"),
            blobs_url: format!("{base}/git/blobs{{/sha}}"),
            branches_url: format!("{base}/branches{{/branch}}"),
            collaborators_url: format!("{base}/collaborators{{/collaborator}}"),
            comments_url: format!("{base}/comments{{/number}}"),
            commits_url: format!("{base}/commits{{/sha}}"),
            compare_url: format!("{base}/compare/{{base}}...{{head}}"),
            contents_url: format!("{base}/contents/{{+path}}"),
            contributors_url: format!("{base}/contributors"),
            deployments_url: format!("{base}/deployments"),
            downloads_url: format!("{base}/downloads"),
            events_url: format!("{base}/events"),
            forks_url: format!("{base}/forks"),
            git_commits_url: format!("{base}/git/commits{{/sha}}"),
            git_refs_url: format!("{base}/git/refs{{/sha}}"),
            git_tags_url: format!("{base}/git/tags{{/sha}}"),
            git_url: format!("git://github.com/{owner}/{name}.git"),
            issue_comment_url: format!("{base}/issues/comments{{/number}}"),
            issue_events_url: format!("{base}/issues/events{{/number}}"),
            issues_url: format!("{base}/issues{{/number}}"),
            keys_url: format!("{base}/keys{{/key_id}}"),
            labels_url: format!("{base}/labels{{/name}}"),
            languages_url: format!("{base}/languages"),
            merges_url: format!("{base}/merges"),
            milestones_url: format!("{base}/milestones{{/number}}"),
            notifications_url: format!("{base}/notifications{{?since,all,participating}}"),
            pulls_url: format!("{base}/pulls{{/number}}"),
            releases_url: format!("{base}/releases{{/id}}"),
            ssh_url: format!("git@github.com:{owner}/{name}.git"),
            stargazers_url: format!("{base}/stargazers"),
            statuses_url: format!("{base}/statuses/{{sha}}"),
            subscribers_url: format!("{base}/subscribers"),
            subscription_url: format!("{base}/subscription"),
            tags_url: format!("{base}/tags"),
            teams_url: format!("{base}/teams"),
            trees_url: format!("{base}/git/trees{{/sha}}"),
            clone_url: format!("https://github.com/{owner}/{name}.git"),
            mirror_url: None,
            hooks_url: format!("{base}/hooks"),
            svn_url: html_base,
            homepage: None,
            language: None,
            forks_count: 0,
            stargazers_count: 0,
            watchers_count: 0,
            size: 0,
            default_branch: "main".to_string(),
            open_issues_count: 0,
            is_template: false,
            topics: Vec::new(),
            has_issues: true,
            has_projects: true,
            has_wiki: true,
            has_pages: false,
            has_downloads: true,
            has_discussions: false,
            archived: false,
            disabled: false,
            visibility: "public".to_string(),
            pushed_at: super::DEFAULT_TIMESTAMP.to_string(),
            created_at: super::DEFAULT_TIMESTAMP.to_string(),
            updated_at: super::DEFAULT_TIMESTAMP.to_string(),
            license: None,
            allow_forking: true,
            web_commit_signoff_required: false,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn clear_description(mut self) -> Self {
        self.description = None;
        self
    }

    pub fn private(mut self, private: bool) -> Self {
        self.private = private;
        self.visibility = if private { "private" } else { "public" }.to_string();
        self
    }

    pub fn stargazers_count(mut self, count: u64) -> Self {
        self.stargazers_count = count;
        self.watchers_count = count;
        self
    }

    pub fn default_branch(mut self, branch: impl Into<String>) -> Self {
        self.default_branch = branch.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Repository;

    #[test]
    fn test_repository_new_defaults() {
        let repo = Repository::new("octocat", "hello-world");
        assert_eq!(repo.name, "hello-world");
        assert_eq!(repo.full_name, "octocat/hello-world");
        assert_eq!(repo.owner.login, "octocat");
        assert!(!repo.private);
        assert_eq!(repo.default_branch, "main");
        assert_eq!(repo.stargazers_count, 0);
        assert!(repo.description.is_none());
    }

    #[test]
    fn test_repository_builder() {
        let repo = Repository::new("test-user", "my-repo")
            .description("A test repository")
            .private(true)
            .stargazers_count(1337)
            .default_branch("develop");

        assert_eq!(repo.description, Some("A test repository".to_string()));
        assert!(repo.private);
        assert_eq!(repo.stargazers_count, 1337);
        assert_eq!(repo.watchers_count, 1337);
        assert_eq!(repo.default_branch, "develop");
    }

    #[test]
    fn test_repository_clear_description() {
        let repo = Repository::new("user", "repo")
            .description("temporary")
            .clear_description();
        assert!(repo.description.is_none());
    }

    #[test]
    fn test_repository_set_private_updates_visibility() {
        let repo = Repository::new("user", "repo").private(true);
        assert!(repo.private);
        assert_eq!(repo.visibility, "private");
    }

    #[test]
    fn test_repository_default_visibility() {
        let repo = Repository::new("user", "repo");
        assert!(!repo.private);
        assert_eq!(repo.visibility, "public");
    }

    #[test]
    fn test_repository_builder_has_hash_id() {
        let repo = Repository::new("user", "repo");
        assert_ne!(repo.id, 0);
        assert!(repo.node_id.starts_with("mock_node_id_"));
    }
}
