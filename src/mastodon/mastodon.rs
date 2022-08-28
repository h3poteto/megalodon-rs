use super::api_client::APIClient;
use super::entities;
use crate::{
    entities as MegalodonEntities, error::Error, megalodon::Megalodon, response::Response,
};
use async_trait::async_trait;

pub struct Mastodon {
    client: APIClient,
    base_url: String,
}

impl Mastodon {
    pub fn new(
        base_url: String,
        access_token: Option<String>,
        user_agent: Option<String>,
    ) -> Mastodon {
        let client = APIClient::new(base_url.clone(), access_token, user_agent);
        Mastodon { client, base_url }
    }
}

#[async_trait]
impl Megalodon for Mastodon {
    async fn verify_account_credentials(
        &self,
    ) -> Result<Response<MegalodonEntities::Account>, Error> {
        let res = self
            .client
            .get::<entities::Account>("/api/v1/accounts/verify_credentials")
            .await?;
        Ok(Response::<MegalodonEntities::Account>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_instance(&self) -> Result<Response<MegalodonEntities::Instance>, Error> {
        let res = self
            .client
            .get::<entities::Instance>("/api/v1/instance")
            .await?;

        Ok(Response::<MegalodonEntities::Instance>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }
}
