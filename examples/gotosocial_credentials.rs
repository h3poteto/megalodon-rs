use megalodon::{entities, error, generator, SNS};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("GOTOSOCIAL_URL") else {
        println!("Specify GOTOSOCIAL_URL!!");
        return
    };
    let Ok(token) = env::var("GOTOSOCIAL_ACCESS_TOKEN") else {
        println!("Specify GOTOSOCIAL_ACCESS_TOKEN!!");
        return
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
    let client = generator(SNS::Gotosocial, url.to_string(), Some(access_token), None);
    let res = client.verify_account_credentials().await?;
    Ok(res.json())
}
