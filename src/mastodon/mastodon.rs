use super::api_client::APIClient;
use crate::{entities::Instance, error::Error, megalodon::Megalodon, response::Response};
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
    async fn get_instance(&self) -> Result<Response<Instance>, Error> {
        self.client.get::<Instance>("/api/v1/instance").await
    }
}
