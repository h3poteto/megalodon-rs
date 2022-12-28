use megalodon::{generator, streaming::Message};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("MASTODON_STREAMING_URL") else {
        println!("Specify MASTODON_STREAMING_URL!!");
        return
    };
    let Ok(token) = env::var("MASTODON_ACCESS_TOKEN") else {
        println!("Specify MASTODON_ACCESS_TOKEN!!");
        return
    };

    streaming(url.as_str(), token).await;
}

async fn streaming(url: &str, access_token: String) {
    let client = generator(
        megalodon::SNS::Mastodon,
        url.to_string(),
        Some(access_token),
        None,
    );
    let streaming = client.user_streaming(url.to_string());

    streaming
        .listen(Box::new(|message| match message {
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
        }))
        .await;
}
