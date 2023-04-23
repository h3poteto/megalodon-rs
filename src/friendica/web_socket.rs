use crate::streaming::{Message, Streaming};
use async_trait::async_trait;

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
        log::error!("Friendica does not support WebSocket")
    }
}
