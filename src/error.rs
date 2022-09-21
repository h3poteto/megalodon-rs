use std::fmt;

pub enum Error {
    ParseError(url::ParseError),
    RequestError(reqwest::Error),
    StandardError(std::io::Error),
    WebSocketError(tungstenite::error::Error),
    JsonError(serde_json::Error),
    OwnError(OwnError),
}

pub struct OwnError {
    url: Option<String>,
    status: Option<u16>,
    message: String,
    kind: Kind,
}

#[derive(Debug)]
pub enum Kind {
    NoImplementedError,
    ParseError,
    HTTPStatusError,
}

impl Error {
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
