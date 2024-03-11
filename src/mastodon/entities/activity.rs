use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Activity {
    week: String,
    statuses: String,
    logins: String,
    registrations: String,
}

impl From<Activity> for MegalodonEntities::Activity {
    fn from(val: Activity) -> Self {
        MegalodonEntities::Activity {
            week: val.week,
            statuses: val.statuses,
            logins: val.logins,
            registrations: val.registrations,
        }
    }
}
