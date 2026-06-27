//! Traits for custom HTTP clients.

use std::future::Future;

/// A trait for HTTP clients used by the downloader.
pub trait HttpClient: Send + Sync + Clone + 'static {
    /// The type of errors returned by this client.
    type Error: std::error::Error + Send + Sync + 'static;

    /// The type of response returned by this client.
    type Response: Response<Error = Self::Error>;

    /// Send a GET request to the specified URL.
    fn get(&self, url: &str) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send;
}

/// A trait representing an HTTP response.
pub trait Response: Send {
    /// The type of errors returned by the chunk reader.
    type Error;

    /// The type representing chunk bytes.
    type Bytes: AsRef<[u8]> + Send;

    /// Get the HTTP status code of the response.
    fn status(&self) -> u16;

    /// Get the content length of the response, if available.
    fn content_length(&self) -> Option<u64>;

    /// Get the next chunk of the response body.
    ///
    /// Returns `Ok(Some(bytes))` if a chunk was successfully read.
    /// Returns `Ok(None)` if the end of the stream was reached.
    fn chunk(&mut self) -> impl Future<Output = Result<Option<Self::Bytes>, Self::Error>> + Send;
}
