use megalodon::{entities, error, generator, SNS};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("GOTOSOCIAL_URL") else {
        println!("Specify GOTOSOCIAL_URL!!");
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
    let client = generator(SNS::Gotosocial, url.to_string(), None, None);
    let res = client.get_instance().await?;
    Ok(res.json())
}
