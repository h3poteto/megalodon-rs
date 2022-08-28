use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct URLs {
    pub streaming_api: String,
}
