use std::thread;
use std::time::Duration;

use super::entities;
use crate::error::{Error, Kind};
use crate::streaming::{Message, Streaming};
use serde::Deserialize;
use tungstenite::protocol::frame::coding::CloseCode;
use tungstenite::{connect, Message as WebSocketMessage};
use url::Url;

const RECONNECT_INTERVAL: u64 = 1000;

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

    fn reconnect(&self, url: &str, callback: Box<dyn Fn(Message)>) {
        thread::sleep(Duration::from_millis(RECONNECT_INTERVAL));
        log::info!("Reconnecting to {}", url);
        self.connect(url, callback)
    }

    fn connect(&self, url: &str, callback: Box<dyn Fn(Message)>) {
        let (mut socket, response) = connect(Url::parse(url).unwrap()).expect("Can't connect");

        log::debug!("Connected to {}", url);
        log::debug!("Response HTTP code: {}", response.status());
        log::debug!("Response contains the following headers:");
        for (ref header, _value) in response.headers() {
            log::debug!("* {}", header);
        }

        loop {
            let msg = socket.read_message().expect("Error reading message");
            if msg.is_ping() {
                let _ = socket
                    .write_message(WebSocketMessage::Pong(Vec::<u8>::new()))
                    .map_err(|e| {
                        log::error!("{:#?}", e);
                        e
                    });
            }
            if msg.is_close() {
                if let WebSocketMessage::Close(Some(close)) = msg {
                    log::warn!("Connection to {} is closed because {}", url, close.code);
                    if close.code != CloseCode::Normal {
                        self.reconnect(url, callback);
                        return;
                    }
                }
                let _ = socket.close(None).map_err(|e| {
                    log::error!("{:#?}", e);
                    e
                });
                return;
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
