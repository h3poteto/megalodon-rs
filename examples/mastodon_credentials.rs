use megalodon::{entities, error, generator, SNS};
use std::env;

#[tokio::main]
async fn main() {
    match env::var("MASTODON_ACCESS_TOKEN") {
        Ok(token) => match verify_credentials("https://fedibird.com", token).await {
            Ok(response) => {
                println!("{:#?}", response);
            }
            Err(err) => {
                println!("{:#?}", err);
            }
        },
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}
async fn verify_credentials(
    url: &str,
    access_token: String,
) -> Result<entities::Account, error::Error> {
    let client = generator(SNS::Mastodon, url.to_string(), Some(access_token), None);
    let res = client.verify_account_credentials().await?;
    Ok(res.json())
}
