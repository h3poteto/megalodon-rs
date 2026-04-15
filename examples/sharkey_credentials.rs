use megalodon::{SNS, entities, error, generator};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("SHARKEY_URL") else {
        println!("Specify SHARKEY_URL!!");
        return;
    };
    let Ok(token) = env::var("SHARKEY_ACCESS_TOKEN") else {
        println!("Specify SHARKEY_ACCESS_TOKEN!!");
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
    let client = generator(SNS::Sharkey, url.to_string(), Some(access_token), None)?;
    let res = client.verify_account_credentials().await?;
    Ok(res.json())
}
