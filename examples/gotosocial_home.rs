use std::env;

use megalodon::{entities, error, generator};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("GOTOSOCIAL_URL") else {
        println!("Specify GOTOSOCIAL_URL!!");
        return;
    };
    let Ok(token) = env::var("GOTOSOCIAL_ACCESS_TOKEN") else {
        println!("Specify GOTOSOCIAL_ACCESS_TOKEN!!");
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
        megalodon::SNS::Gotosocial,
        url.to_string(),
        Some(access_token),
        None,
    )?;
    let res = client.get_home_timeline(None).await?;

    Ok(res.json())
}
