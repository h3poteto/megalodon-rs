use megalodon::{generator, streaming::Message};
use std::env;

fn main() {
    env_logger::init();
    match env::var("PLEROMA_ACCESS_TOKEN") {
        Ok(token) => {
            streaming("wss://pleroma.io", token);
        }
        Err(err) => {
            log::error!("{:#?}", err);
        }
    }
}

fn streaming(url: &str, access_token: String) {
    let client = generator(
        megalodon::SNS::Pleroma,
        url.to_string(),
        Some(access_token),
        None,
    );
    let streaming = client.public_streaming(url.to_string());

    streaming.listen(Box::new(|message| match message {
        Message::Update(mes) => {
            println!("{:#?}", mes);
        }
        Message::Notification(mes) => {
            println!("{:#?}", mes);
        }
        Message::Conversation(mes) => {
            println!("{:#?}", mes);
        }
        Message::Delete(mes) => {
            println!("{:#?}", mes);
        }
        Message::Heartbeat() => {
            println!("heartbeat");
        }
    }))
}
