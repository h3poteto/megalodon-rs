use megalodon::{generator, streaming::Message};
use std::env;

fn main() {
    match env::var("MASTODON_ACCESS_TOKEN") {
        Ok(token) => {
            streaming("wss://streaming.fedibird.com", token);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

fn streaming(url: &str, access_token: String) {
    let client = generator(
        megalodon::SNS::Mastodon,
        url.to_string(),
        Some(access_token),
        None,
    );
    let streaming = client.user_streaming(url.to_string());

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
            println!("message is deleted: {}", mes);
        }
        Message::Heartbeat() => {
            println!("heartbeat");
        }
    }));
}
