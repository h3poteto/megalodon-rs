use megalodon::{entities, error, generator, SNS};
use std::env;

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
    match verify_credentials(url.as_str(), token).await {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}
async fn verify_credentials(
    url: &str,
    access_token: String,
) -> Result<entities::Account, error::Error> {
    let client = generator(
        SNS::Pixelfed,
        reqwest::Client::new(),
        url.to_string(),
        Some(access_token),
    );
    let res = client.verify_account_credentials().await?;
    Ok(res.json())
}
