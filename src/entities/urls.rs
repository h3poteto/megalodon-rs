use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct URLs {
    streaming_api: String,
}
