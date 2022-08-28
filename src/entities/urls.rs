use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct URLs {
    streaming_api: String,
}
