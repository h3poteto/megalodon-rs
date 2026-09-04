use std::env;

use megalodon::{entities, error, generator};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("PIXELFED_URL") else {
        println!("Specify PIXELFED_URL!!");
        return;
    };
    let Ok(token) = env::var("PIXELFED_ACCESS_TOKEN") else {
        println!("Specify PIXELFED_ACCESS_TOKEN!!");
        return;
    };

    let res = home_timeline(url.as_str(), token).await;
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn home_timeline(
    url: &str,
    access_token: String,
) -> Result<Vec<entities::Status>, error::Error> {
    let client = generator(
        megalodon::SNS::Pixelfed,
        reqwest::Client::new(),
        url.to_string(),
        Some(access_token),
    );
    let res = client.get_public_timeline(None).await?;

    Ok(res.json())
}
