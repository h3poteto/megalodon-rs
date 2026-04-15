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

    let res = get_relationship(url.as_str(), token, "ab2w2m8cp8").await;
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn get_relationship(
    url: &str,
    access_token: String,
    id: &str,
) -> Result<Vec<entities::Relationship>, error::Error> {
    let client = generator(
        megalodon::SNS::Sharkey,
        url.to_string(),
        Some(access_token),
        None,
    )?;
    let res = client.get_relationships([id.to_string()].to_vec()).await?;
    Ok(res.json())
}
