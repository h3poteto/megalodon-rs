use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Application {
    name: String,
    website: Option<String>,
    vapid_key: Option<String>,
}

impl Into<MegalodonEntities::Application> for Application {
    fn into(self) -> MegalodonEntities::Application {
        MegalodonEntities::Application {
            name: self.name,
            website: self.website,
            vapid_key: self.vapid_key,
        }
    }
}
