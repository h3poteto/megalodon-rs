//! Response modules
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

/// Response struct for API response.
#[derive(Debug, Clone)]
pub struct Response<T> {
    /// Parsed json object.
    pub json: T,
    /// Status code of the response.
    pub status: u16,
    /// Status text of the response.
    pub status_text: String,
    /// Headers of the response.
    pub header: HeaderMap,
}

impl<T> Response<T> {
    /// Create a new Response struct.
    pub fn new(json: T, status: u16, status_text: String, header: HeaderMap) -> Response<T> {
        Self {
            json,
            status,
            status_text,
            header,
        }
    }

    /// Create a new Response struct from reqwest::Response.
    pub async fn from_reqwest(response: reqwest::Response) -> Result<Response<T>, reqwest::Error>
    where
        T: DeserializeOwned + Debug,
    {
        let header = response.headers().clone();
        let status_code = response.status();
        let json = response.json::<T>().await?;

        Ok(Self {
            json,
            status: status_code.as_u16(),
            status_text: status_code.as_str().to_string(),
            header,
        })
    }

    /// Get json object.
    pub fn json(&self) -> T
    where
        T: Clone,
    {
        self.json.clone()
    }
}
