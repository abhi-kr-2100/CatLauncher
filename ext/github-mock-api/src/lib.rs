use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::Router;
use tokio::sync::RwLock;
use tokio::sync::mpsc;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to send shutdown signal to server: {0}")]
    ShutdownError(#[from] tokio::sync::mpsc::error::TrySendError<()>),

    #[error("Failed to wait for server to stop: {0}")]
    JoinError(#[from] tokio::task::JoinError),
}

pub type Result<T> = std::result::Result<T, Error>;

mod api;
mod release;
mod commit;
mod repository;
mod util;

pub use release::Release;
pub use commit::Commit;
pub use repository::Repository;

pub(crate) type RepoKey = (String, String);

#[derive(Clone, Default)]
pub(crate) struct AppState {
    pub(crate) repositories: Arc<RwLock<HashMap<RepoKey, Repository>>>,
    pub(crate) releases: Arc<RwLock<HashMap<RepoKey, Vec<Release>>>>,
    pub(crate) commits: Arc<RwLock<HashMap<RepoKey, Vec<Commit>>>>,
}

impl AppState {
    pub async fn add_release(&self, owner: &str, repo: &str, release: Release) {
        let key = (owner.to_lowercase(), repo.to_lowercase());
        let mut releases = self.releases.write().await;
        releases.entry(key).or_default().push(release);
    }

    pub async fn add_commit(&self, owner: &str, repo: &str, commit: Commit) {
        let key = (owner.to_lowercase(), repo.to_lowercase());
        let mut commits = self.commits.write().await;
        commits.entry(key).or_default().push(commit);
    }

    pub async fn add_repository(&self, repository: Repository) {
        let key = (
            repository.owner.login.to_lowercase(),
            repository.name.to_lowercase(),
        );
        let mut repositories = self.repositories.write().await;
        repositories.insert(key, repository);
    }
}

/// A mock GitHub API server that can be used for testing.
///
/// # Example
///
/// ```no_run
/// use github_mock_api::{MockServer, Error};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Error> {
///     let server = MockServer::start().await?;
///     let addr = server.uri();
///     println!("Server running on {}", addr);
///     // The server will be automatically shut down when `server` is dropped
///     Ok(())
/// }
/// ```
pub struct MockServer {
    address: SocketAddr,
    shutdown_sender: Option<mpsc::Sender<()>>,
    server_handle: Option<tokio::task::JoinHandle<()>>,
    state: AppState,
}

impl MockServer {
    /// Start a new mock server on 127.0.0.1 with a randomly available port.
    pub async fn start() -> Result<Self> {
        let host = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        Self::start_on(host, 0).await
    }

    /// Start a new mock server on the specified host and port.
    /// Use port 0 for a randomly available port.
    pub async fn start_on(host: IpAddr, port: u16) -> Result<Self> {
        let listener = tokio::net::TcpListener::bind((host, port))
            .await?;

        let address = listener.local_addr()?;

        let state = AppState::default();
        let app = Router::new()
            .route(
                "/repos/{owner}/{repo}",
                get(|Path((owner, repo)), State(state): State<AppState>| {
                    repository::get_repository(owner, repo, state)
                }),
            )
            .route(
                "/repos/{owner}/{repo}/releases",
                get(
                    |Path((owner, repo)),
                     Query(pagination),
                     State(state): State<AppState>| {
                        release::list_releases(owner, repo, pagination, state)
                    },
                ),
            )
            .route(
                "/repos/{owner}/{repo}/releases/latest",
                get(|Path((owner, repo)), State(state): State<AppState>| {
                    release::get_latest_release(owner, repo, state)
                }),
            )
            .route(
                "/repos/{owner}/{repo}/releases/tags/{tag}",
                get(|Path((owner, repo, tag)), State(state): State<AppState>| {
                    release::get_release_by_tag(owner, repo, tag, state)
                }),
            )
            .route(
                "/repos/{owner}/{repo}/releases/{release_id}",
                get(
                    |Path((owner, repo, release_id)), State(state): State<AppState>| {
                        release::get_release(owner, repo, release_id, state)
                    },
                ),
            )
            .route(
                "/repos/{owner}/{repo}/commits",
                get(
                    |Path((owner, repo)),
                     Query(pagination),
                     State(state): State<AppState>| {
                        commit::list_commits(owner, repo, pagination, state)
                    },
                ),
            )
            .route(
                "/repos/{owner}/{repo}/commits/{sha}",
                get(
                    |Path((owner, repo, sha)), State(state): State<AppState>| {
                        commit::get_commit(owner, repo, sha, state)
                    },
                ),
            )
            .with_state(state.clone());

        let (shutdown_sender, mut shutdown_receiver) = mpsc::channel(1);

        let server_handle = tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    shutdown_receiver.recv().await;
                    tracing::info!("Shutdown signal received");
                })
                .await
            {
                tracing::error!("Server failed: {}", e);
            }
        });

        Ok(Self {
            address,
            shutdown_sender: Some(shutdown_sender),
            server_handle: Some(server_handle),
            state,
        })
    }

    /// Get the full URI of the running server (e.g., "http://127.0.0.1:3000").
    pub fn uri(&self) -> String {
        format!("http://{}", self.address)
    }

    /// Register a mocked release with the server.
    pub async fn add_release(&self, owner: &str, repo: &str, release: Release) {
        self.state.add_release(owner, repo, release).await;
    }

    /// Register a mocked commit with the server.
    pub async fn add_commit(&self, owner: &str, repo: &str, commit: Commit) {
        self.state.add_commit(owner, repo, commit).await;
    }

    /// Register a mocked repository with the server.
    pub async fn add_repository(&self, repository: Repository) {
        self.state.add_repository(repository).await;
    }

    /// Manually stop the server.
    pub async fn stop(&mut self) -> Result<()> {
        self.stop_inner()?;
        if let Some(handle) = self.server_handle.take() {
            handle.await?;
        }
        Ok(())
    }

    fn stop_inner(&mut self) -> Result<()> {
        if let Some(sender) = self.shutdown_sender.take() {
            match sender.try_send(()) {
                Ok(_) => {}
                Err(e) => match e {
                    tokio::sync::mpsc::error::TrySendError::Closed(_) => {
                        // Receiver was dropped (e.g., server crashed), ignore
                    }
                    _ => return Err(e.into()),
                },
            }
        }
        Ok(())
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        let _ = self.stop_inner();
        if let Some(handle) = self.server_handle.take() {
            handle.abort();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_starts_default() -> Result<()> {
        let server = MockServer::start().await?;
        let addr = server.address;

        // Verify the server is actually running by connecting to it
        tokio::net::TcpStream::connect(addr).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_server_starts_on_host_and_port() -> Result<()> {
        let host: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let server = MockServer::start_on(host, 3030).await?;
        let addr = server.address;

        // Verify the server is actually running by connecting to it
        tokio::net::TcpStream::connect(addr).await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_server_stop_explicitly() -> Result<()> {
        let mut server = MockServer::start().await?;
        let addr = server.address;

        server.stop().await?;

        // Connection should fail now
        let result = tokio::net::TcpStream::connect(addr).await;
        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_server_stop_on_drop() -> Result<()> {
        let server = MockServer::start().await?;
        let addr = server.address;

        // Drop the server - should trigger shutdown via Drop impl
        drop(server);

        // Give the server a moment to shut down
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Connection should fail now
        let result = tokio::net::TcpStream::connect(addr).await;
        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_uri_default() -> Result<()> {
        let server = MockServer::start().await?;

        let uri = server.uri();
        assert!(uri.starts_with("http://127.0.0.1:"));

        let port = server.address.port();
        assert!(uri.ends_with(&format!(":{}", port)));

        Ok(())
    }

    #[tokio::test]
    async fn test_uri_with_port() -> Result<()> {
        let host = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let port = 17893; // Use a high port number unlikely to be in use
        let server = MockServer::start_on(host, port).await?;
        let uri = server.uri();

        assert_eq!(uri, "http://0.0.0.0:17893");

        Ok(())
    }

    #[tokio::test]
    async fn test_stop_idempotency() -> Result<()> {
        let mut server = MockServer::start().await?;

        // First stop should succeed
        server.stop().await?;

        // Second stop should also succeed (idempotent)
        server.stop().await?;

        // Third stop should also succeed (idempotent)
        server.stop().await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_stop_after_server_crash() -> Result<()> {
        let mut server = MockServer::start().await?;

        // Abort the server handle to simulate a crash (this drops the shutdown_receiver)
        if let Some(handle) = server.server_handle.take() {
            handle.abort();
            // Wait for the task to actually be aborted
            let _ = handle.await;
        }

        // Calling stop on an already stopped server should not lead to an error
        let result = server.stop().await;
        assert!(result.is_ok());
        Ok(())
    }
}
