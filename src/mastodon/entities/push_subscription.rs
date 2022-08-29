use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PushSubscription {
    id: String,
    endpoint: String,
    server_key: String,
    alerts: Alerts,
}

#[derive(Debug, Deserialize, Clone)]
struct Alerts {
    follow: bool,
    favourite: bool,
    mention: bool,
    reblog: bool,
    poll: bool,
}

impl Into<MegalodonEntities::PushSubscription> for PushSubscription {
    fn into(self) -> MegalodonEntities::PushSubscription {
        MegalodonEntities::PushSubscription {
            id: self.id,
            endpoint: self.endpoint,
            server_key: self.server_key,
            alerts: self.alerts.into(),
        }
    }
}

impl Into<MegalodonEntities::push_subscription::Alerts> for Alerts {
    fn into(self) -> MegalodonEntities::push_subscription::Alerts {
        MegalodonEntities::push_subscription::Alerts {
            follow: self.follow,
            favourite: self.favourite,
            mention: self.mention,
            reblog: self.reblog,
            poll: self.poll,
        }
    }
}
