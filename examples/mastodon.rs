use megalodon::{entities, error, mastodon::Mastodon, Megalodon};
use std::env;

#[tokio::main]
async fn main() {
    match instance("https://fedibird.com").await {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

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

async fn instance(url: &str) -> Result<entities::Instance, error::Error> {
    let client = Mastodon::new(url.to_string(), None, None);
    let res = client.get_instance().await?;
    Ok(res.json())
}

async fn verify_credentials(
    url: &str,
    access_token: String,
) -> Result<entities::Account, error::Error> {
    let client = Mastodon::new(url.to_string(), Some(access_token), None);
    let res = client.verify_account_credentials().await?;
    Ok(res.json())
}
