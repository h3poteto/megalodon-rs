use std::fmt;
use std::thread;
use std::time::Duration;

use super::entities;
use crate::default::DEFAULT_UA;
use crate::error::{Error, Kind};
use crate::streaming::{Message, Streaming};
use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        client::IntoClientRequest,
        http::StatusCode,
        protocol::{frame::coding::CloseCode, Message as WebSocketMessage},
        Error as WebSocketError,
    },
    MaybeTlsStream, WebSocketStream,
};
use tracing::{debug, error, info, warn};
use url::Url;
use uuid::Uuid;

const RECONNECT_INTERVAL: u64 = 5000;
const READ_MESSAGE_TIMEOUT_SECONDS: u64 = 60;

#[derive(Debug, Clone)]
pub struct WebSocket {
    url: String,
    channel: String,
    list_id: Option<String>,
    access_token: Option<String>,
    user_agent: String,
    channel_id: String,
}

#[derive(Deserialize)]
struct RawMessage {
    r#type: String,
    body: MessageBody,
}

#[derive(Deserialize)]
struct MessageBody {
    id: String,
    r#type: String,
    body: serde_json::Value,
}

impl WebSocket {
    pub fn new(
        url: String,
        channel: String,
        list_id: Option<String>,
        access_token: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        let ua: String;
        match user_agent {
            Some(agent) => ua = agent,
            None => ua = DEFAULT_UA.to_string(),
        }
        Self {
            url,
            channel,
            list_id,
            access_token,
            user_agent: ua,
            channel_id: Uuid::new_v4().to_string(),
        }
    }

    fn parse(&self, message: WebSocketMessage) -> Result<Message, Error> {
        if message.is_ping() || message.is_pong() {
            Ok(Message::Heartbeat())
        } else if message.is_text() {
            let text = message.to_text()?;
            let Ok(mes) = serde_json::from_str::<RawMessage>(text) else {
                return Ok(Message::Heartbeat());
            };
            if mes.r#type != "channel" || mes.body.id != self.channel_id {
                return Ok(Message::Heartbeat());
            }
            match &*mes.body.r#type {
                "note" => {
                    let res = serde_json::from_value::<entities::Note>(mes.body.body.clone())
                        .map_err(|e| {
                            error!(
                                "failed to parse note: {}\n{}",
                                e.to_string(),
                                &mes.body.body
                            );
                            e
                        })?;
                    Ok(Message::Update(res.into()))
                }
                "notification" => {
                    let res =
                        serde_json::from_value::<entities::Notification>(mes.body.body.clone())
                            .map_err(|e| {
                                error!(
                                    "failed to parse notification: {}\n{}",
                                    e.to_string(),
                                    &mes.body.body
                                );
                                e
                            })?;
                    Ok(Message::Notification(res.into()))
                }
                "mention" => {
                    let res = serde_json::from_value::<entities::Note>(mes.body.body.clone())
                        .map_err(|e| {
                            error!(
                                "failed to parse note: {}\n{}",
                                e.to_string(),
                                &mes.body.body
                            );
                            e
                        })?;
                    Ok(Message::Conversation(res.into()))
                }
                unknown => {
                    warn!("Unknown body type message is received: {}", unknown);
                    Ok(Message::Heartbeat())
                }
            }
        } else {
            Err(Error::new_own(
                String::from("Receiving message is not ping, pong or text"),
                Kind::ParseError,
                None,
                None,
                None,
            ))
        }
    }

    async fn connect(&self, url: &str, callback: Box<dyn Fn(Message) + Send + Sync>) {
        loop {
            match self.do_connect(url, &callback).await {
                Ok(()) => {
                    info!("connection for {} is  closed", url);
                    return;
                }
                Err(err) => match err.kind {
                    InnerKind::ConnectionError
                    | InnerKind::SocketReadError
                    | InnerKind::UnusualSocketCloseError
                    | InnerKind::TimeoutError => {
                        thread::sleep(Duration::from_millis(RECONNECT_INTERVAL));
                        info!("Reconnecting to {}", url);
                        continue;
                    }
                    InnerKind::UnauthorizedError => {
                        info!("Unauthorized so give up");
                        return;
                    }
                },
            }
        }
    }

    async fn do_connect(
        &self,
        url: &str,
        callback: &Box<dyn Fn(Message) + Send + Sync>,
    ) -> Result<(), InnerError> {
        let mut req = Url::parse(url)
            .unwrap()
            .into_client_request()
            .map_err(|e| {
                error!("Failed to parse url: {}", e);
                InnerError::new(InnerKind::ConnectionError)
            })?;
        req.headers_mut()
            .insert("User-Agent", self.user_agent.parse().unwrap());
        let (socket, response) = connect_async(req).await.map_err(|e| {
            error!("Failed to connect: {}", e);
            match e {
                WebSocketError::Http(response) => match response.status() {
                    StatusCode::UNAUTHORIZED => InnerError::new(InnerKind::UnauthorizedError),
                    _ => InnerError::new(InnerKind::ConnectionError),
                },
                _ => InnerError::new(InnerKind::ConnectionError),
            }
        })?;

        debug!("Connected to {}", url);
        debug!("Response HTTP code: {}", response.status());
        debug!("Response contains the following headers:");
        for (ref header, _value) in response.headers() {
            debug!("* {}", header);
        }

        let mut socket = self.connect_channel(socket).await;

        loop {
            let res = tokio::time::timeout(
                Duration::from_secs(READ_MESSAGE_TIMEOUT_SECONDS),
                socket.next(),
            )
            .await
            .map_err(|e| {
                error!("Timeout reading message: {}", e);
                InnerError::new(InnerKind::TimeoutError)
            })?;
            let Some(r) = res else {
                warn!("Response is empty");
                continue;
            };
            let msg = r.map_err(|e| {
                error!("Failed to read message: {}", e);
                InnerError::new(InnerKind::SocketReadError)
            })?;
            if msg.is_ping() {
                let _ = socket
                    .send(WebSocketMessage::Pong(Vec::<u8>::new()))
                    .await
                    .map_err(|e| {
                        error!("{:#?}", e);
                        e
                    });
            }
            if msg.is_close() {
                let _ = socket.close(None).await.map_err(|e| {
                    error!("{:#?}", e);
                    e
                });
                if let WebSocketMessage::Close(Some(close)) = msg {
                    warn!("Connection to {} is closed because {}", url, close.code);
                    if close.code != CloseCode::Normal {
                        return Err(InnerError::new(InnerKind::UnusualSocketCloseError));
                    }
                }
                return Ok(());
            }
            match self.parse(msg) {
                Ok(message) => {
                    callback(message);
                }
                Err(err) => {
                    warn!("{}", err);
                }
            }
        }
    }

    async fn connect_channel(
        &self,
        mut socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
    ) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        match self.channel.as_ref() {
            "conversation" => {
                let data = json!({
                    "type": "connect",
                    "body": {
                        "channel": "main",
                        "id": self.channel_id,
                    },
                });
                let _ = socket
                    .send(WebSocketMessage::Text(data.to_string()))
                    .await
                    .map_err(|e| {
                        error!("{:#?}", e);
                        e
                    });
            }
            "user" => {
                let data = json!({
                    "type": "connect",
                    "body": {
                        "channel": "main",
                        "id": self.channel_id,
                    },
                });
                let _ = socket
                    .send(WebSocketMessage::Text(data.to_string()))
                    .await
                    .map_err(|e| {
                        error!("{:#?}", e);
                        e
                    });
                let home = json!({
                    "type": "connect",
                    "body": {
                        "channel": "homeTimeline",
                        "id": self.channel_id,
                        "params": {
                            "withReplies": false
                        }
                    },
                });
                debug!("Sending {:?}", &home);
                let _ = socket
                    .send(WebSocketMessage::Text(home.to_string()))
                    .await
                    .map_err(|e| {
                        error!("{:#?}", e);
                        e
                    });
            }
            "list" => {
                let data = json!({
                    "type": "connect",
                    "body": {
                        "channel": "userList",
                        "id": self.channel_id,
                        "params": {
                            "listId": self.list_id,
                            "withReplies": false
                        }
                    },
                });
                debug!("Sending {:?}", &data);
                let _ = socket
                    .send(WebSocketMessage::Text(data.to_string()))
                    .await
                    .map_err(|e| {
                        error!("{:#?}", e);
                        e
                    });
            }
            channel => {
                let data = json!({
                    "type": "connect",
                    "body": {
                        "channel": channel,
                        "id": self.channel_id,
                        "params": {
                            "withReplies": false
                        }
                    },
                });
                debug!("Sending {:?}", &data);
                let _ = socket
                    .send(WebSocketMessage::Text(data.to_string()))
                    .await
                    .map_err(|e| {
                        error!("{:#?}", e);
                        e
                    });
            }
        }
        socket
    }
}

#[async_trait]
impl Streaming for WebSocket {
    async fn listen(&self, callback: Box<dyn Fn(Message) + Send + Sync>) {
        let mut parameter = Vec::<String>::new();
        if let Some(access_token) = &self.access_token {
            parameter.push(format!("i={}", access_token));
        }
        let mut url = self.url.clone();
        url = url + "?" + parameter.join("&").as_str();

        self.connect(url.as_str(), callback).await;
    }
}

#[derive(thiserror::Error)]
#[error("{kind}")]
struct InnerError {
    kind: InnerKind,
}

#[derive(Debug, thiserror::Error)]
enum InnerKind {
    #[error("connection error")]
    ConnectionError,
    #[error("socket read error")]
    SocketReadError,
    #[error("unusual socket close error")]
    UnusualSocketCloseError,
    #[error("timeout error")]
    TimeoutError,
    #[error("unauthorized error")]
    UnauthorizedError,
}

impl InnerError {
    pub fn new(kind: InnerKind) -> Self {
        Self { kind }
    }
}

impl fmt::Debug for InnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("megalodon::mastodon::web_socket::InnerError");

        builder.field("kind", &self.kind);
        builder.finish()
    }
}
