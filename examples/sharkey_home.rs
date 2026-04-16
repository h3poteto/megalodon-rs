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

    let res = timeline(url.as_str(), token).await;
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn timeline(url: &str, access_token: String) -> Result<Vec<entities::Status>, error::Error> {
    let client = generator(
        megalodon::SNS::Sharkey,
        url.to_string(),
        Some(access_token),
        None,
    )?;
    let res = client.get_home_timeline(None).await?;

    Ok(res.json())
}
