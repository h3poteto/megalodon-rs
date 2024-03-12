use megalodon::{entities, error, generator, SNS};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("FIREFISH_URL") else {
        println!("Specify FIREFISH_URL!!");
        return;
    };
    match emojis(url.as_str()).await {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn emojis(url: &str) -> Result<Vec<entities::Emoji>, error::Error> {
    let client = generator(SNS::Firefish, url.to_string(), None, None);
    let res = client.get_instance_custom_emojis().await?;
    Ok(res.json())
}
