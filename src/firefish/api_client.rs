use crate::default::DEFAULT_UA;
use crate::error::{Error as MegalodonError, Kind};
use crate::response::Response;
use reqwest::header::HeaderMap;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct APIClient {
    access_token: Option<String>,
    base_url: String,
    client: reqwest::Client,
}

impl APIClient {
    pub fn new(
        base_url: String,
        access_token: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Self, MegalodonError> {
        let ua: String;
        match user_agent {
            Some(agent) => ua = agent,
            None => ua = DEFAULT_UA.to_string(),
        }

        let client = reqwest::Client::builder().user_agent(ua).build()?;

        Ok(Self {
            access_token,
            base_url,
            client,
        })
    }

    pub async fn get<T>(
        &self,
        path: &str,
        headers: Option<HeaderMap>,
    ) -> Result<Response<T>, MegalodonError>
    where
        T: DeserializeOwned + Debug,
    {
        let url_str = format!("{}{}", self.base_url, path);
        let url = Url::parse(&*url_str)?;

        let mut req = self.client.get(url);
        if let Some(token) = &self.access_token {
            req = req.bearer_auth(token);
        }
        if let Some(headers) = headers {
            req = req.headers(headers);
        }

        let res = req.send().await?;
        let res_headers = res.headers().clone();
        let status = res.status();
        match status {
            reqwest::StatusCode::OK
            | reqwest::StatusCode::CREATED
            | reqwest::StatusCode::ACCEPTED
            | reqwest::StatusCode::NO_CONTENT => {
                let res = Response::<T>::from_reqwest(res).await?;
                Ok(res)
            }
            reqwest::StatusCode::PARTIAL_CONTENT => Err(MegalodonError::new_own(
                String::from("The requested resource is still being processed"),
                Kind::HTTPPartialContentError,
                Some(url_str),
                Some(status.as_u16()),
                Some(res_headers),
            )),
            _ => match res.text().await {
                Ok(text) => Err(MegalodonError::new_own(
                    text,
                    Kind::HTTPStatusError,
                    Some(url_str),
                    Some(status.as_u16()),
                    Some(res_headers),
                )),
                Err(_err) => Err(MegalodonError::new_own(
                    "Unknown error".to_string(),
                    Kind::HTTPStatusError,
                    Some(url_str),
                    Some(status.as_u16()),
                    Some(res_headers),
                )),
            },
        }
    }

    pub async fn post<T>(
        &self,
        path: &str,
        params: &HashMap<&str, Value>,
        headers: Option<HeaderMap>,
    ) -> Result<Response<T>, MegalodonError>
    where
        T: DeserializeOwned + Debug,
    {
        let url_str = format!("{}{}", self.base_url, path);
        let url = Url::parse(&*url_str)?;

        let mut req = self.client.post(url);
        if let Some(token) = &self.access_token {
            req = req.bearer_auth(token);
        }
        if let Some(headers) = headers {
            req = req.headers(headers);
        }

        let res = req.json(params).send().await?;
        let res_headers = res.headers().clone();
        let status = res.status();
        match status {
            reqwest::StatusCode::OK
            | reqwest::StatusCode::CREATED
            | reqwest::StatusCode::ACCEPTED
            | reqwest::StatusCode::NO_CONTENT => {
                let res = Response::<T>::from_reqwest(res).await?;
                Ok(res)
            }
            _ => match res.text().await {
                Ok(text) => Err(MegalodonError::new_own(
                    text,
                    Kind::HTTPStatusError,
                    Some(url_str),
                    Some(status.as_u16()),
                    Some(res_headers),
                )),
                Err(_err) => Err(MegalodonError::new_own(
                    "Unknown error".to_string(),
                    Kind::HTTPStatusError,
                    Some(url_str),
                    Some(status.as_u16()),
                    Some(res_headers),
                )),
            },
        }
    }

    pub async fn post_multipart<T>(
        &self,
        path: &str,
        params: reqwest::multipart::Form,
        headers: Option<HeaderMap>,
    ) -> Result<Response<T>, MegalodonError>
    where
        T: DeserializeOwned + Debug,
    {
        let url_str = format!("{}{}", self.base_url, path);
        let url = Url::parse(&*url_str)?;

        let mut req = self.client.post(url);
        if let Some(token) = &self.access_token {
            req = req.bearer_auth(token);
        }
        if let Some(headers) = headers {
            req = req.headers(headers);
        }

        let res = req.multipart(params).send().await?;
        let res_headers = res.headers().clone();
        let status = res.status();
        match status {
            reqwest::StatusCode::OK
            | reqwest::StatusCode::CREATED
            | reqwest::StatusCode::ACCEPTED
            | reqwest::StatusCode::NO_CONTENT => {
                let res = Response::<T>::from_reqwest(res).await?;
                Ok(res)
            }
            _ => match res.text().await {
                Ok(text) => Err(MegalodonError::new_own(
                    text,
                    Kind::HTTPStatusError,
                    Some(url_str),
                    Some(status.as_u16()),
                    Some(res_headers),
                )),
                Err(_err) => Err(MegalodonError::new_own(
                    "Unknown error".to_string(),
                    Kind::HTTPStatusError,
                    Some(url_str),
                    Some(status.as_u16()),
                    Some(res_headers),
                )),
            },
        }
    }
}

pub static DEFAULT_SCOPES: &'static [&str] = &[
    "read:account",
    "write:account",
    "read:blocks",
    "write:blocks",
    "read:drive",
    "write:drive",
    "read:favorites",
    "write:favorites",
    "read:following",
    "write:following",
    "read:mutes",
    "write:mutes",
    "write:notes",
    "read:notifications",
    "write:notifications",
    "read:reactions",
    "write:reactions",
    "write:votes",
];
