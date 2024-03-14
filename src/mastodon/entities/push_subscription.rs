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

impl From<PushSubscription> for MegalodonEntities::PushSubscription {
    fn from(val: PushSubscription) -> Self {
        MegalodonEntities::PushSubscription {
            id: val.id,
            endpoint: val.endpoint,
            server_key: val.server_key,
            alerts: val.alerts.into(),
        }
    }
}

impl From<Alerts> for MegalodonEntities::push_subscription::Alerts {
    fn from(val: Alerts) -> Self {
        MegalodonEntities::push_subscription::Alerts {
            follow: val.follow,
            favourite: val.favourite,
            mention: val.mention,
            reblog: val.reblog,
            poll: val.poll,
        }
    }
}
