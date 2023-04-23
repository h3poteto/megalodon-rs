use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct URLs {
    streaming_api: String,
}

impl Into<MegalodonEntities::URLs> for URLs {
    fn into(self) -> MegalodonEntities::URLs {
        MegalodonEntities::URLs {
            streaming_api: self.streaming_api,
        }
    }
}
