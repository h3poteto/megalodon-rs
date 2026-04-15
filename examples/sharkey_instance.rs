use megalodon::{SNS, entities, error, generator};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("SHARKEY_URL") else {
        println!("Specify SHARKEY_URL!!");
        return;
    };
    match instance(url.as_str()).await {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn instance(url: &str) -> Result<entities::Instance, error::Error> {
    let client = generator(SNS::Sharkey, url.to_string(), None, None)?;
    let res = client.get_instance().await?;
    Ok(res.json())
}
