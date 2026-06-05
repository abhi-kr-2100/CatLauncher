use std::net::IpAddr;
use std::sync::OnceLock;
use tokio::runtime::Runtime;
use github_mock_api::Error as MockApiError;

#[derive(Debug, Clone, Copy)]
pub enum CommonError {
    Io,
    Shutdown,
    Join,
    InvalidHost,
}

impl From<MockApiError> for CommonError {
    fn from(err: MockApiError) -> Self {
        match err {
            MockApiError::Io(_) => CommonError::Io,
            MockApiError::ShutdownError(_) => CommonError::Shutdown,
            MockApiError::JoinError(_) => CommonError::Join,
        }
    }
}

pub fn runtime() -> Result<&'static Runtime, CommonError> {
    static RUNTIME: OnceLock<Result<Runtime, CommonError>> = OnceLock::new();
    RUNTIME
        .get_or_init(|| Runtime::new().map_err(|_| CommonError::Io))
        .as_ref()
        .map_err(|err| *err)
}

pub fn parse_host(host: &str) -> Result<IpAddr, CommonError> {
    host.parse::<IpAddr>()
        .map_err(|_| CommonError::InvalidHost)
}
