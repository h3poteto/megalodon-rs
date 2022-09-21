use crate::entities as MegalodonEntities;

pub trait Streaming {
    fn listen(&self, callback: Box<dyn Fn(Message)>);
}

pub enum Message {
    Update(MegalodonEntities::Status),
    Notification(MegalodonEntities::Notification),
    Conversation(MegalodonEntities::Conversation),
    Delete(String),
    Heartbeat(),
}
