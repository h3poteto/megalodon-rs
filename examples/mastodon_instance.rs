use megalodon::{entities, error, generator, SNS};

#[tokio::main]
async fn main() {
    match instance("https://fedibird.com").await {
        Ok(response) => {
            println!("{:#?}", response);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn instance(url: &str) -> Result<entities::Instance, error::Error> {
    let client = generator(SNS::Mastodon, url.to_string(), None, None);
    let res = client.get_instance().await?;
    Ok(res.json())
}
