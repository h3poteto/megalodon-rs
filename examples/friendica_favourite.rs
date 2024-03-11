use std::env;

use megalodon::{entities, error, generator};

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("FRIENDICA_URL") else {
        println!("Specify FRIENDICA_URL!!");
        return;
    };
    let Ok(token) = env::var("FRIENDICA_ACCESS_TOKEN") else {
        println!("Specify FRIENDICA_ACCESS_TOKEN!!");
        return;
    };
    let Ok(status_id) = env::var("STATUS_ID") else {
        println!("Specify STATUS_ID!!");
        return;
    };

    let res = favourite_status(url.as_str(), token, status_id).await;
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn favourite_status(
    url: &str,
    access_token: String,
    status_id: String,
) -> Result<entities::Status, error::Error> {
    let client = generator(
        megalodon::SNS::Friendica,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client.favourite_status(status_id).await?;

    Ok(res.json())
}
