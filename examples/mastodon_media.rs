use std::env;

use megalodon::{entities, error, generator};

#[tokio::main]
async fn main() {
    match env::var("MASTODON_ACCESS_TOKEN") {
        Ok(token) => {
            let file_path = "./sample.jpg".to_string();
            let res = upload_media("https://fedibird.com", token.to_owned(), file_path).await;
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

async fn upload_media(
    url: &str,
    access_token: String,
    file_path: String,
) -> Result<entities::Attachment, error::Error> {
    let client = generator(
        megalodon::SNS::Mastodon,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client.upload_media(file_path, None).await?;
    Ok(res.json())
}
