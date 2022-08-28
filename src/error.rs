use std::fmt;

pub struct Error {
    url: Option<String>,
    status: Option<u16>,
    message: String,
    kind: Kind,
}

#[derive(Debug)]
pub enum Kind {
    ParseError,
    RequestError,
}

impl Error {
    pub fn new(message: String) -> Error {
        Self {
            url: None,
            status: None,
            message,
            kind: Kind::ParseError,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        let url;
        match err.url() {
            Some(u) => url = Some(u.as_str().to_string()),
            None => url = None,
        }
        let status;
        match err.status() {
            Some(s) => status = Some(s.as_u16()),
            None => status = None,
        }

        Self {
            url,
            status,
            message: err.to_string(),
            kind: Kind::RequestError,
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Self {
            url: None,
            status: None,
            message: err.to_string(),
            kind: Kind::ParseError,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            Kind::ParseError => f.write_str("parse error")?,
            Kind::RequestError => f.write_str("request error")?,
        }

        write!(f, "message {}", self.message)?;

        if let Some(ref url) = self.url {
            write!(f, "for URL {}", url)?;
        }

        if let Some(ref status) = self.status {
            write!(f, "status code {}", status)?;
        }

        Ok(())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("megalodon::Error");

        builder.field("kind", &self.kind);
        builder.field("message", &self.message);

        if let Some(ref url) = self.url {
            builder.field("url", url);
        }

        builder.finish()
    }
}
