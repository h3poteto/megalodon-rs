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
        megalodon::SNS::Friendica,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client.get_home_timeline(None).await?;

    Ok(res.json())
}
