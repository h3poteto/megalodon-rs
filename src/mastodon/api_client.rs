use crate::default::DEFAULT_UA;
use crate::error::Error as MegalodonError;
use crate::response::Response;
use reqwest::header::HeaderMap;
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fmt::Debug;

pub struct APIClient {
    access_token: Option<String>,
    base_url: String,
    user_agent: String,
}

impl APIClient {
    pub fn new(base_url: String, access_token: Option<String>, user_agent: Option<String>) -> Self {
        let ua: String;
        match user_agent {
            Some(agent) => ua = agent,
            None => ua = DEFAULT_UA.to_string(),
        }

        Self {
            access_token,
            base_url,
            user_agent: ua,
        }
    }

    pub async fn get<T>(
        &self,
        path: &str,
        headers: Option<HeaderMap>,
    ) -> Result<Response<T>, MegalodonError>
    where
        T: DeserializeOwned + Debug,
    {
        let url = format!("{}{}", self.base_url, path);
        let url = Url::parse(&*url)?;
        let client = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .build()?;

        let mut req = client.get(url);
        if let Some(token) = &self.access_token {
            req = req.bearer_auth(token);
        }
        if let Some(headers) = headers {
            req = req.headers(headers);
        }

        let res = req.send().await?;
        let res = Response::<T>::from_reqwest(res).await?;
        Ok(res)
    }

    pub async fn post<T>(
        &self,
        path: &str,
        params: &HashMap<&str, String>,
        headers: Option<HeaderMap>,
    ) -> Result<Response<T>, MegalodonError>
    where
        T: DeserializeOwned + Debug,
    {
        let url = format!("{}{}", self.base_url, path);
        let url = Url::parse(&*url)?;
        let client = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .build()?;

        let mut req = client.post(url);
        if let Some(token) = &self.access_token {
            req = req.bearer_auth(token);
        }
        if let Some(headers) = headers {
            req = req.headers(headers);
        }

        let res = req.form(params).send().await?;
        let res = Response::<T>::from_reqwest(res).await?;
        Ok(res)
    }

    pub async fn put<T>(
        &self,
        path: &str,
        params: &HashMap<&str, String>,
        headers: Option<HeaderMap>,
    ) -> Result<Response<T>, MegalodonError>
    where
        T: DeserializeOwned + Debug,
    {
        let url = format!("{}{}", self.base_url, path);
        let url = Url::parse(&*url)?;
        let client = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .build()?;

        let mut req = client.put(url);
        if let Some(token) = &self.access_token {
            req = req.bearer_auth(token);
        }
        if let Some(headers) = headers {
            req = req.headers(headers);
        }

        let res = req.form(params).send().await?;
        let res = Response::<T>::from_reqwest(res).await?;
        Ok(res)
    }

    pub async fn patch<T>(
        &self,
        path: &str,
        params: &HashMap<&str, String>,
        headers: Option<HeaderMap>,
    ) -> Result<Response<T>, MegalodonError>
    where
        T: DeserializeOwned + Debug,
    {
        let url = format!("{}{}", self.base_url, path);
        let url = Url::parse(&*url)?;
        let client = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .build()?;

        let mut req = client.patch(url);
        if let Some(token) = &self.access_token {
            req = req.bearer_auth(token);
        }
        if let Some(headers) = headers {
            req = req.headers(headers);
        }

        let res = req.form(params).send().await?;
        let res = Response::<T>::from_reqwest(res).await?;
        Ok(res)
    }

    pub async fn delete<T>(
        &self,
        path: &str,
        params: &HashMap<&str, String>,
        headers: Option<HeaderMap>,
    ) -> Result<Response<T>, MegalodonError>
    where
        T: DeserializeOwned + Debug,
    {
        let url = format!("{}{}", self.base_url, path);
        let url = Url::parse(&*url)?;
        let client = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .build()?;

        let mut req = client.delete(url);
        if let Some(token) = &self.access_token {
            req = req.bearer_auth(token);
        }
        if let Some(headers) = headers {
            req = req.headers(headers);
        }

        let res = req.form(params).send().await?;
        let res = Response::<T>::from_reqwest(res).await?;
        Ok(res)
    }
}
