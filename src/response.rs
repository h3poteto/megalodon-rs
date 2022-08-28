use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

pub struct Response<T> {
    json: T,
    status: u16,
    status_text: String,
    header: HeaderMap,
}

impl<T> Response<T> {
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

    pub fn json(&self) -> T
    where
        T: Clone,
    {
        self.json.clone()
    }
}
