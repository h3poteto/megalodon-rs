use crate::streaming::{Message, Streaming};
use async_trait::async_trait;
use tracing::error;

#[derive(Debug, Clone)]
pub struct WebSocket {}

impl WebSocket {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Streaming for WebSocket {
    async fn listen(&self, _callback: Box<dyn Fn(Message) + Send + Sync>) {
        error!("Friendica does not support WebSocket")
    }
}
