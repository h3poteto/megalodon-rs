use crate::error::Error as MegalodonError;
use crate::response::Response;
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub struct APIClient {
    access_token: Option<String>,
    base_url: String,
    user_agent: String,
}

static DEFAULT_UA: &str = "megalodon";

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

    pub async fn get<T>(&self, path: &str) -> Result<Response<T>, MegalodonError>
    where
        T: DeserializeOwned + Debug,
    {
        let url = format!("{}{}", self.base_url, path);
        let url = Url::parse(&*url);

        match url {
            Err(err) => Err(err.into()),
            Ok(url) => {
                let res = reqwest::get(url).await?;
                let res = Response::<T>::from_reqwest(res).await?;
                Ok(res)
            }
        }
    }
}
