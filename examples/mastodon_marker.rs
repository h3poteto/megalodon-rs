use megalodon::{entities, error, generator};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("MASTODON_URL") else {
        println!("Specify MASTODON_URL!!");
        return;
    };
    let Ok(token) = env::var("MASTODON_ACCESS_TOKEN") else {
        println!("Specify MASTODON_ACCESS_TOKEN!!");
        return;
    };

    let res = get_markers(url.as_str(), token).await;
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn get_markers(url: &str, access_token: String) -> Result<entities::Marker, error::Error> {
    let client = generator(
        megalodon::SNS::Mastodon,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client
        .get_markers(vec![String::from("home"), String::from("notifications")])
        .await?;
    Ok(res.json())
}
