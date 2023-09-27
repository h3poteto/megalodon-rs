use std::env;

use megalodon::{entities, error, generator};

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("FIREFISH_URL") else {
        println!("Specify FIREFISH_URL!!");
        return;
    };
    let Ok(token) = env::var("FIREFISH_ACCESS_TOKEN") else {
        println!("Specify FIREFISH_ACCESS_TOKEN!!");
        return;
    };

    let res = search(url.as_str(), token).await;
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn search(url: &str, access_token: String) -> Result<entities::Results, error::Error> {
    let client = generator(
        megalodon::SNS::Firefish,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client.search(String::from("h3poteto"), None).await?;

    Ok(res.json())
}
