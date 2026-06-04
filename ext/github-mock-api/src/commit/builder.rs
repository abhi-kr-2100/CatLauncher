use std::sync::atomic::{AtomicU64, Ordering};

use crate::util::hash;

use super::types::{
    Commit, CommitDetail, CommitTree, GitUser, SimpleUser, Verification,
};

fn generate_sha() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let h1 = crate::util::hash(&format!("commit_high:{n}"));
    let h2 = crate::util::hash(&format!("commit_low:{n}"));
    format!("{:016x}{:016x}{:08x}", h1, h2, h1.wrapping_add(h2) & 0xFFFF_FFFF)
}

impl Commit {
    pub fn new(owner: &str, repo: &str) -> Self {
        let sha = generate_sha();
        let id = hash(&format!("commit:{owner}/{repo}:{sha}"));
        let base = format!("https://api.github.com/repos/{owner}/{repo}");
        let html_base = format!("https://github.com/{owner}/{repo}");
        let commit_url = format!("{base}/git/commits/{sha}");
        let commit_html_url = format!("{html_base}/commit/{sha}");

        Self {
            url: format!("{base}/commits/{sha}"),
            sha: sha.clone(),
            node_id: format!("mock_commit_node_{id}"),
            html_url: commit_html_url,
            comments_url: format!("{base}/commits/{sha}/comments"),
            commit: CommitDetail {
                url: commit_url,
                author: Some(GitUser {
                    name: "Mona Octocat".to_string(),
                    email: "mona@github.com".to_string(),
                    date: super::DEFAULT_TIMESTAMP.to_string(),
                }),
                committer: Some(GitUser {
                    name: "Mona Octocat".to_string(),
                    email: "mona@github.com".to_string(),
                    date: super::DEFAULT_TIMESTAMP.to_string(),
                }),
                message: String::new(),
                comment_count: 0,
                tree: CommitTree {
                    sha: generate_sha(),
                    url: format!("{base}/git/trees/{sha}"),
                },
                verification: Verification {
                    verified: false,
                    reason: "unsigned".to_string(),
                    payload: None,
                    signature: None,
                    verified_at: None,
                },
            },
            author: Some(SimpleUser::new(owner)),
            committer: Some(SimpleUser::new(owner)),
            parents: Vec::new(),
            owner: owner.to_string(),
            repo: repo.to_string(),
        }
    }

    pub fn sha(mut self, sha: impl Into<String>) -> Self {
        let sha = sha.into();
        let owner = &self.owner;
        let repo = &self.repo;
        let base = format!("https://api.github.com/repos/{owner}/{repo}");
        let html_base = format!("https://github.com/{owner}/{repo}");
        let commit_url = format!("{base}/git/commits/{sha}");
        let commit_html_url = format!("{html_base}/commit/{sha}");
        self.sha = sha.clone();
        self.url = format!("{base}/commits/{sha}");
        self.html_url = commit_html_url;
        self.comments_url = format!("{base}/commits/{sha}/comments");
        self.commit.url = commit_url;
        self.commit.tree.url = format!("{base}/git/trees/{sha}");
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.commit.message = message.into();
        self
    }

    pub fn author_name(mut self, name: impl Into<String>) -> Self {
        if let Some(ref mut author) = self.commit.author {
            author.name = name.into();
        }
        self
    }

    pub fn author_email(mut self, email: impl Into<String>) -> Self {
        if let Some(ref mut author) = self.commit.author {
            author.email = email.into();
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Commit;

    #[test]
    fn test_commit_new_defaults() {
        let commit = Commit::new("octocat", "hello-world");
        assert_eq!(commit.owner, "octocat");
        assert_eq!(commit.repo, "hello-world");
        assert_eq!(commit.sha.len(), 40);
        assert!(commit.sha.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(commit.node_id.starts_with("mock_commit_node_"));
        assert!(commit.commit.message.is_empty());
        assert!(commit.parents.is_empty());
    }

    #[test]
    fn test_commit_unique_shas() {
        let c1 = Commit::new("user", "repo");
        let c2 = Commit::new("user", "repo");
        assert_ne!(c1.sha, c2.sha);
    }

    #[test]
    fn test_commit_builder() {
        let commit = Commit::new("test-user", "my-repo")
            .sha("abc123def456")
            .message("A test commit\n\nWith a body")
            .author_name("Test User")
            .author_email("test@example.com");

        assert_eq!(commit.sha, "abc123def456");
        assert_eq!(commit.commit.message, "A test commit\n\nWith a body");
        assert_eq!(commit.commit.author.as_ref().map(|a| &a.name), Some(&"Test User".to_string()));
        assert_eq!(commit.commit.author.as_ref().map(|a| &a.email), Some(&"test@example.com".to_string()));
    }

    #[test]
    fn test_commit_sha_updates_urls() {
        let commit = Commit::new("owner", "repo").sha("customsha");
        assert_eq!(commit.sha, "customsha");
        assert!(commit.url.ends_with("/commits/customsha"));
        assert!(commit.html_url.ends_with("/commit/customsha"));
        assert!(commit.comments_url.ends_with("/commits/customsha/comments"));
        assert!(commit.commit.url.ends_with("/git/commits/customsha"));
    }
}
