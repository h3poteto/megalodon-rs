//! Streaming modules
use crate::entities as MegalodonEntities;
use async_trait::async_trait;
use std::future::Future;
use std::pin::Pin;

/// Streaming interface to listen message.
#[async_trait]
pub trait Streaming {
    /// Whether WebSocket streaming is supported or not.
    fn is_supported(&self) -> bool;
    /// Start listening stream messages. When receive a message, the callback function will be called.
    async fn listen(
        &self,
        callback: Box<
            dyn Fn(Message) -> Pin<Box<dyn Future<Output = ()> + Send>>
                + Send
                + Sync
                + 'async_trait,
        >,
    );
}

/// Stream message definitions.
#[derive(Debug, Clone)]
pub enum Message {
    /// Update message for `update` event.
    Update(MegalodonEntities::Status),
    /// Notification message for `notification` evnet.
    Notification(MegalodonEntities::Notification),
    /// Conversation message for `conversation` event.
    Conversation(MegalodonEntities::Conversation),
    /// Delete message for `delete` event.
    Delete(String),
    /// StatusUpdate message of `status.update` event.
    StatusUpdate(MegalodonEntities::Status),
    /// Heartbeat for streaming connection.
    Heartbeat(),
}
