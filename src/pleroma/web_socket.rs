use std::fmt;
use std::ops::Add;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::entities;
use crate::error::{Error, Kind};
use crate::streaming::{Message, Streaming};
use chrono::Utc;
use serde::Deserialize;
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::protocol::CloseFrame;
use tungstenite::{connect, Message as WebSocketMessage};
use url::Url;

const RECONNECT_INTERVAL: u64 = 1000;
const READ_MESSAGE_TIMEOUT_SECONDS: i64 = 60;

#[derive(Debug, Clone)]
pub struct WebSocket {
    url: String,
    stream: String,
    params: Option<Vec<String>>,
    access_token: Option<String>,
}

#[derive(Deserialize)]
struct RawMessage {
    event: String,
    payload: String,
}

impl WebSocket {
    pub fn new(
        url: String,
        stream: String,
        params: Option<Vec<String>>,
        access_token: Option<String>,
    ) -> Self {
        Self {
            url,
            stream,
            params,
            access_token,
        }
    }

    fn parse(&self, message: WebSocketMessage) -> Result<Message, Error> {
        if message.is_ping() || message.is_pong() {
            Ok(Message::Heartbeat())
        } else if message.is_text() {
            let text = message.to_text()?;
            let mes = serde_json::from_str::<RawMessage>(text)?;
            match &*mes.event {
                "update" => {
                    let res =
                        serde_json::from_str::<entities::Status>(&mes.payload).map_err(|e| {
                            log::error!(
                                "failed to parse status: {}\n{}",
                                e.to_string(),
                                &mes.payload
                            );
                            e
                        })?;
                    Ok(Message::Update(res.into()))
                }
                "notification" => {
                    let res = serde_json::from_str::<entities::Notification>(&mes.payload)
                        .map_err(|e| {
                            log::error!(
                                "failed to parse notification: {}\n{}",
                                e.to_string(),
                                &mes.payload
                            );
                            e
                        })?;
                    Ok(Message::Notification(res.into()))
                }
                "conversation" => {
                    let res = serde_json::from_str::<entities::Conversation>(&mes.payload)
                        .map_err(|e| {
                            log::error!(
                                "failed to parse conversation: {}\n{}",
                                e.to_string(),
                                &mes.payload
                            );
                            e
                        })?;
                    Ok(Message::Conversation(res.into()))
                }
                "delete" => Ok(Message::Delete(mes.payload)),
                event => Err(Error::new_own(
                    format!("Unknown event is received: {}", event),
                    Kind::ParseError,
                    None,
                    None,
                )),
            }
        } else {
            Err(Error::new_own(
                String::from("Receiving message is not ping, pong or text"),
                Kind::ParseError,
                None,
                None,
            ))
        }
    }

    fn connect(&self, url: &str, callback: Box<dyn Fn(Message)>) {
        loop {
            match self.do_connect(url, &callback) {
                Ok(()) => {
                    log::info!("connection for {} is  closed", url);
                    return;
                }
                Err(err) => match err.kind {
                    InnerKind::ConnectionError
                    | InnerKind::SocketReadError
                    | InnerKind::UnusualSocketCloseError => {
                        thread::sleep(Duration::from_millis(RECONNECT_INTERVAL));
                        log::info!("Reconnecting to {}", url);
                        continue;
                    }
                },
            }
        }
    }

    fn do_connect(&self, url: &str, callback: &Box<dyn Fn(Message)>) -> Result<(), InnerError> {
        let (socket, response) = connect(Url::parse(url).unwrap()).map_err(|e| {
            log::error!("Failed to connect: {}", e);
            InnerError::new(InnerKind::ConnectionError)
        })?;

        log::debug!("Connected to {}", url);
        log::debug!("Response HTTP code: {}", response.status());
        log::debug!("Response contains the following headers:");
        for (ref header, _value) in response.headers() {
            log::debug!("* {}", header);
        }

        let last_received = Arc::new(Mutex::new(Utc::now()));
        let last_received_check = Arc::clone(&last_received);
        let socket = Arc::new(Mutex::new(socket));
        let socket_check = Arc::clone(&socket);

        let stop = Arc::new(AtomicBool::new(false));
        let stop_check = Arc::clone(&stop);

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(10));

            if stop_check.load(Ordering::Relaxed) {
                return;
            }

            let ts = last_received_check.lock().unwrap();
            log::debug!("last received: {}", ts);
            let diff = Utc::now() - ts.add(chrono::Duration::seconds(READ_MESSAGE_TIMEOUT_SECONDS));
            if diff > chrono::Duration::seconds(0) {
                log::warn!("closing connection because timeout");
                socket_check
                    .lock()
                    .unwrap()
                    .close(Some(CloseFrame {
                        code: CloseCode::Again,
                        reason: std::borrow::Cow::Borrowed("Timeout"),
                    }))
                    .unwrap();
                return;
            }
        });

        loop {
            let msg = socket.lock().unwrap().read_message().map_err(|e| {
                log::error!("Failed to read message: {}", e);
                stop.store(true, Ordering::Relaxed);
                InnerError::new(InnerKind::SocketReadError)
            })?;
            let mut ts = last_received.lock().unwrap();
            *ts = Utc::now();
            drop(ts);
            if msg.is_ping() {
                let _ = socket
                    .lock()
                    .unwrap()
                    .write_message(WebSocketMessage::Pong(Vec::<u8>::new()))
                    .map_err(|e| {
                        log::error!("{:#?}", e);
                        e
                    });
            }
            if msg.is_close() {
                stop.store(true, Ordering::Relaxed);
                let _ = socket.lock().unwrap().close(None).map_err(|e| {
                    log::error!("{:#?}", e);
                    e
                });
                if let WebSocketMessage::Close(Some(close)) = msg {
                    log::warn!("Connection to {} is closed because {}", url, close.code);
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
                    log::warn!("{}", err);
                }
            }
        }
    }
}

impl Streaming for WebSocket {
    fn listen(&self, callback: Box<dyn Fn(Message)>) {
        let mut parameter = Vec::<String>::from([format!("stream={}", self.stream)]);
        if let Some(access_token) = &self.access_token {
            parameter.push(format!("access_token={}", access_token));
        }
        if let Some(mut params) = self.params.clone() {
            parameter.append(&mut params);
        }
        let mut url = self.url.clone();
        url = url + "?" + parameter.join("&").as_str();

        self.connect(url.as_str(), callback);
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
}

impl InnerError {
    pub fn new(kind: InnerKind) -> Self {
        Self { kind }
    }
}

impl fmt::Debug for InnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut builder = f.debug_struct("megalodon::pleroma::web_socket::InnerError");

        builder.field("kind", &self.kind);
        builder.finish()
    }
}
