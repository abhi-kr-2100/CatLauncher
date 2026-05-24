use std::path::Path;

use github_mock_api::Repository;
use serde::Deserialize;

/// Standard seed file names loaded from a data directory.
pub const REPOSITORIES_FILE: &str = "repositories.json";

/// A repository entry in `repositories.json`, using the same fields as the builder API.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RepositorySeed {
    pub owner: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub private: Option<bool>,
    #[serde(default)]
    pub stargazers_count: Option<u64>,
    #[serde(default)]
    pub default_branch: Option<String>,
}

impl From<RepositorySeed> for Repository {
    fn from(seed: RepositorySeed) -> Self {
        let mut repo = Repository::new(&seed.owner, &seed.name);
        if let Some(description) = seed.description {
            repo = repo.description(description);
        }
        if let Some(private) = seed.private {
            repo = repo.private(private);
        }
        if let Some(count) = seed.stargazers_count {
            repo = repo.stargazers_count(count);
        }
        if let Some(branch) = seed.default_branch {
            repo = repo.default_branch(branch);
        }
        repo
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SeedError {
    #[error("failed to read seed file {path}: {source}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse seed file {path}: {source}")]
    Parse {
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

/// Load repositories from `{data_dir}/repositories.json` when that file exists.
///
/// Returns `Ok(None)` if the data directory or seed file is missing.
pub fn load_repositories(data_dir: &Path) -> Result<Option<Vec<Repository>>, SeedError> {
    let path = data_dir.join(REPOSITORIES_FILE);
    if !path.is_file() {
        return Ok(None);
    }
    let contents = std::fs::read_to_string(&path).map_err(|source| SeedError::Read {
        path: path.display().to_string(),
        source,
    })?;
    let seeds: Vec<RepositorySeed> =
        serde_json::from_str(&contents).map_err(|source| SeedError::Parse {
            path: path.display().to_string(),
            source,
        })?;
    Ok(Some(seeds.into_iter().map(Repository::from).collect()))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn seed_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("seed")
    }

    #[test]
    fn test_load_repositories_from_seed_dir() -> Result<(), SeedError> {
        let repos = load_repositories(&seed_dir())?.expect("seed file present");
        assert_eq!(repos.len(), 1);
        let repo = &repos[0];
        assert_eq!(repo.owner.login, "octocat");
        assert_eq!(repo.name, "hello-world");
        assert_eq!(
            repo.description.as_deref(),
            Some("This is a mocked repository!")
        );
        assert!(!repo.private);
        assert_eq!(repo.stargazers_count, 1337);
        assert_eq!(repo.default_branch, "main");
        Ok(())
    }

    #[test]
    fn test_load_repositories_missing_dir() -> Result<(), SeedError> {
        assert!(load_repositories(Path::new("/nonexistent/seed/dir"))?.is_none());
        Ok(())
    }

    #[test]
    fn test_load_repositories_missing_file() -> Result<(), SeedError> {
        let dir = std::env::temp_dir().join(format!(
            "github-mock-api-seed-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).map_err(|source| SeedError::Read {
            path: dir.display().to_string(),
            source,
        })?;
        assert!(load_repositories(&dir)?.is_none());
        let _ = std::fs::remove_dir_all(&dir);
        Ok(())
    }

    #[test]
    fn test_repository_seed_deserialize_minimal() {
        let json = r#"{"owner":"user","name":"repo"}"#;
        let seed: RepositorySeed = match serde_json::from_str(json) {
            Ok(seed) => seed,
            Err(err) => panic!("expected valid json: {err}"),
        };
        let repo = Repository::from(seed);
        assert_eq!(repo.owner.login, "user");
        assert_eq!(repo.name, "repo");
        assert!(repo.description.is_none());
        assert!(!repo.private);
    }
}
