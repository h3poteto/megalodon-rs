use crate::error::{Error as MegalodonError, Kind};
use crate::http::HttpClient;
use crate::response::Response;
use reqwest::header::HeaderMap;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct APIClient {
    client: Box<dyn HttpClient>,
    access_token: Option<String>,
    base_url: String,
}

impl APIClient {
    pub fn new(
        client: Box<dyn HttpClient>,
        base_url: String,
        access_token: Option<String>,
    ) -> Self {
        Self {
            client,
            access_token,
            base_url,
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
        let url_str = format!("{}{}", self.base_url, path);
        let url = Url::parse(&*url_str)?;

        let mut req = reqwest::Request::new(reqwest::Method::GET, url);
        if let Some(token) = &self.access_token {
            req.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {token}").try_into()?,
            );
        }
        if let Some(headers) = headers {
            req.headers_mut().extend(headers);
        }

        let res = self.client.request(req).await?;
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

        let mut req = reqwest::Request::new(reqwest::Method::POST, url);
        if let Some(token) = &self.access_token {
            req.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {token}").try_into()?,
            );
        }
        if let Some(headers) = headers {
            req.headers_mut().extend(headers);
        }

        let res = self
            .client
            .request(crate::http::set_json_body(req, params)?)
            .await?;
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

        let mut req = reqwest::Request::new(reqwest::Method::POST, url);
        if let Some(token) = &self.access_token {
            req.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {token}").try_into()?,
            );
        }
        if let Some(headers) = headers {
            req.headers_mut().extend(headers);
        }

        let res = self
            .client
            .request(crate::http::set_multipart_body(req, params)?)
            .await?;
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

    pub async fn put<T>(
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

        let mut req = reqwest::Request::new(reqwest::Method::PUT, url);
        if let Some(token) = &self.access_token {
            req.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {token}").try_into()?,
            );
        }
        if let Some(headers) = headers {
            req.headers_mut().extend(headers);
        }

        let res = self
            .client
            .request(crate::http::set_json_body(req, params)?)
            .await?;
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

    pub async fn put_multipart<T>(
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

        let mut req = reqwest::Request::new(reqwest::Method::PUT, url);
        if let Some(token) = &self.access_token {
            req.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {token}").try_into()?,
            );
        }
        if let Some(headers) = headers {
            req.headers_mut().extend(headers);
        }

        let res = self
            .client
            .request(crate::http::set_multipart_body(req, params)?)
            .await?;
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

    pub async fn patch<T>(
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

        let mut req = reqwest::Request::new(reqwest::Method::PATCH, url);
        if let Some(token) = &self.access_token {
            req.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {token}").try_into()?,
            );
        }
        if let Some(headers) = headers {
            req.headers_mut().extend(headers);
        }

        let res = self
            .client
            .request(crate::http::set_json_body(req, params)?)
            .await?;
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

    pub async fn delete<T>(
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

        let mut req = reqwest::Request::new(reqwest::Method::DELETE, url);
        if let Some(token) = &self.access_token {
            req.headers_mut().insert(
                reqwest::header::AUTHORIZATION,
                format!("Bearer {token}").try_into()?,
            );
        }
        if let Some(headers) = headers {
            req.headers_mut().extend(headers);
        }

        let res = self
            .client
            .request(crate::http::set_json_body(req, params)?)
            .await?;
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
