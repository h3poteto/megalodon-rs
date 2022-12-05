//! Own errors
use std::fmt;

/// Possible megalodon errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// ParseError from [`url::ParseError`].
    /// This error will be raised when provided URL is invalid.
    #[error(transparent)]
    ParseError(#[from] url::ParseError),
    /// RequestError from [`reqwest::Error`].
    /// This error will be raised when the request is invalid or failed to parse the response in reqwest.
    #[error(transparent)]
    RequestError(#[from] reqwest::Error),
    /// StandardError from [`std::io::Error`].
    /// This error will be raised when some standard error has occur.
    #[error(transparent)]
    StandardError(#[from] std::io::Error),
    /// WebSocketError from [`tungstenite::error::Error`].
    /// This error will be raised when tungstenite WebSocket raises an error.
    #[error(transparent)]
    WebSocketError(#[from] tungstenite::error::Error),
    /// JsonError from [`serde_json::Error`].
    /// This error will be raised when failed to parse some json.
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),
    /// OwnError is megalodon own errors.
    #[error(transparent)]
    OwnError(#[from] OwnError),
}

/// Megalodon own errors.
#[derive(thiserror::Error)]
#[error("{kind}: {message} {} {}", .url.as_ref().map(AsRef::as_ref).unwrap_or(""), .status.map(|u| u.to_string()).unwrap_or("".to_string()))]
pub struct OwnError {
    pub url: Option<String>,
    pub status: Option<u16>,
    pub message: String,
    pub kind: Kind,
}

/// Error kind of [`OwnError`].
#[derive(Debug, thiserror::Error)]
pub enum Kind {
    /// The implementation is not found.
    /// When this error is raised, the method has not yet implemented.
    #[error("no implemented error")]
    NoImplementedError,
    /// Failed to parse something.
    #[error("parse error")]
    ParseError,
    /// The request responds http response with error code.
    #[error("http status error")]
    HTTPStatusError,
    /// The request is not completed error.
    #[error("partial content error")]
    HTTPPartialContentError,
}

impl Error {
    /// Create a new [`OwnError`] struct.
    pub fn new_own(message: String, kind: Kind, url: Option<String>, status: Option<u16>) -> Error {
        Error::OwnError(OwnError {
            message,
            kind,
            url,
            status,
        })
    }
}

impl fmt::Debug for OwnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("megalodon::OwnError");

        builder.field("kind", &self.kind);
        builder.field("message", &self.message);

        if let Some(ref url) = self.url {
            builder.field("url", url);
        }
        if let Some(ref status) = self.status {
            builder.field("status", status);
        }

        builder.finish()
    }
}
