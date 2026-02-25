use std::future::Future;
use std::pin::Pin;

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
    fn is_supported(&self) -> bool {
        false
    }

    async fn listen(
        &self,
        _callback: Box<
            dyn Fn(Message) -> Pin<Box<dyn Future<Output = ()> + Send>>
                + Send
                + Sync
                + 'async_trait,
        >,
    ) {
        error!("Friendica does not support WebSocket")
    }
}
