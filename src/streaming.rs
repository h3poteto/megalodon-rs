//! Streaming modules
use crate::entities as MegalodonEntities;

/// Streaming interface to listen message.
pub trait Streaming {
    /// Start listening stream messages. When receive a message, the callback function will be called.
    fn listen(&self, callback: Box<dyn Fn(Message)>);
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
    /// Heartbeat for streaming connection.
    Heartbeat(),
}
