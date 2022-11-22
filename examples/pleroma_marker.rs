use std::env;

use megalodon::{entities, error, generator};

#[tokio::main]
async fn main() {
    env_logger::init();
    match env::var("PLEROMA_ACCESS_TOKEN") {
        Ok(token) => {
            let res = get_markers("https://pleroma.io", token).await;
            match res {
                Ok(res) => {
                    println!("{:#?}", res);
                }
                Err(err) => {
                    println!("{:#?}", err);
                }
            }
        }
        Err(err) => {
            println!("{:#?}", err)
        }
    }
}

async fn get_markers(url: &str, access_token: String) -> Result<entities::Marker, error::Error> {
    let client = generator(
        megalodon::SNS::Pleroma,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client
        .get_markers(vec![String::from("notifications")])
        .await?;
    Ok(res.json())
}
