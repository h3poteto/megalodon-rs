use megalodon::{entities, error, generator};
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

    let res = get_note(url.as_str(), token).await;
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn get_note(url: &str, access_token: String) -> Result<entities::Status, error::Error> {
    let client = generator(
        megalodon::SNS::Sharkey,
        url.to_string(),
        Some(access_token),
        None,
    )?;
    let res = client.get_status("akw9yq4031".to_string()).await?;
    Ok(res.json())
}
