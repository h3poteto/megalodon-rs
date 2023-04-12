use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Activity {
    week: String,
    statuses: String,
    logins: String,
    registrations: String,
}

impl Into<MegalodonEntities::Activity> for Activity {
    fn into(self) -> MegalodonEntities::Activity {
        MegalodonEntities::Activity {
            week: self.week,
            statuses: self.statuses,
            logins: self.logins,
            registrations: self.registrations,
        }
    }
}
