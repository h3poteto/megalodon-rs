use megalodon::detector;
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("MASTODON_URL") else {
        println!("Specify MASTODON_URL!!");
        return;
    };
    let sns = detector(url.as_str()).await;
    println!("{:#?}", sns);
}
