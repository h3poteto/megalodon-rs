//! Own errors
use std::fmt;

/// Possible megalodon errors.
pub enum Error {
    /// ParseError from [`url::ParseError`].
    /// This error will be raised when provided URL is invalid.
    ParseError(url::ParseError),
    /// RequestError from [`reqwest::Error`].
    /// This error will be raised when the request is invalid or failed to parse the response in reqwest.
    RequestError(reqwest::Error),
    /// StandardError from [`std::io::Error`].
    /// This error will be raised when some standard error has occur.
    StandardError(std::io::Error),
    /// WebSocketError from [`tungstenite::error::Error`].
    /// This error will be raised when tungstenite WebSocket raises an error.
    WebSocketError(tungstenite::error::Error),
    /// JsonError from [`serde_json::Error`].
    /// This error will be raised when failed to parse some json.
    JsonError(serde_json::Error),
    /// OwnError is megalodon own errors.
    OwnError(OwnError),
}

/// Megalodon own errors.
pub struct OwnError {
    url: Option<String>,
    status: Option<u16>,
    message: String,
    kind: Kind,
}

/// Error kind of [`OwnError`].
#[derive(Debug)]
pub enum Kind {
    /// The implementation is not found.
    /// When this error is raised, the method has not yet implemented.
    NoImplementedError,
    /// Failed to parse something.
    ParseError,
    /// The request responds http response with error code.
    HTTPStatusError,
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

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::RequestError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::ParseError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::StandardError(err)
    }
}

impl From<tungstenite::error::Error> for Error {
    fn from(err: tungstenite::error::Error) -> Error {
        Error::WebSocketError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::JsonError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OwnError(err) => err.fmt(f),
            Error::ParseError(err) => err.fmt(f),
            Error::RequestError(err) => err.fmt(f),
            Error::StandardError(err) => err.fmt(f),
            Error::WebSocketError(err) => err.fmt(f),
            Error::JsonError(err) => err.fmt(f),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::OwnError(err) => err.fmt(f),
            Error::ParseError(err) => err.fmt(f),
            Error::RequestError(err) => err.fmt(f),
            Error::StandardError(err) => err.fmt(f),
            Error::WebSocketError(err) => err.fmt(f),
            Error::JsonError(err) => err.fmt(f),
        }
    }
}

impl fmt::Display for OwnError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            Kind::NoImplementedError => f.write_str("no implemented error: ")?,
            Kind::ParseError => f.write_str("parse error: ")?,
            Kind::HTTPStatusError => f.write_str("http status error: ")?,
        }

        write!(f, "message {}", self.message)?;

        if let Some(ref url) = self.url {
            write!(f, "for URL {}", url)?;
        }
        if let Some(ref status) = self.status {
            write!(f, "status {}", status)?;
        }

        Ok(())
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
