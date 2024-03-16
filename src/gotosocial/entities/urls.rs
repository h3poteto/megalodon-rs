use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct URLs {
    streaming_api: String,
}

impl From<URLs> for MegalodonEntities::URLs {
    fn from(val: URLs) -> MegalodonEntities::URLs {
        MegalodonEntities::URLs {
            streaming_api: val.streaming_api,
        }
    }
}
