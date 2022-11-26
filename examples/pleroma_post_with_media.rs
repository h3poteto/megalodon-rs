use megalodon::{
    entities::{self, StatusVisibility},
    error, generator,
    megalodon::PostStatusInputOptions,
};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("PLEROMA_URL") else {
        println!("Specify PLEROMA_URL!!");
        return
    };
    let Ok(token) = env::var("PLEROMA_ACCESS_TOKEN") else {
        println!("Specify PLEROMA_ACCESS_TOKEN!!");
        return
    };

    let file_path = "./sample.jpg".to_string();
    let res = upload_media(url.as_str(), token.to_owned(), file_path.to_string()).await;
    match res {
        Ok(res) => {
            let media_id_1 = res.id;
            let file_path = "./sample2.jpg".to_string();
            let res = upload_media(url.as_str(), token.to_owned(), file_path).await;
            match res {
                Ok(res) => {
                    let media_id_2 = res.id;
                    let media_ids = vec![media_id_1, media_id_2];
                    match post_status(
                        url.as_str(),
                        token,
                        "Post with attached media",
                        Some(media_ids),
                    )
                    .await
                    {
                        Ok(res) => {
                            println!("{:#?}", res);
                        }
                        Err(err) => {
                            println!("{:#?}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("{:#?}", err);
                }
            }
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn upload_media(
    url: &str,
    access_token: String,
    file_path: String,
) -> Result<entities::Attachment, error::Error> {
    let client = generator(
        megalodon::SNS::Pleroma,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client.upload_media(file_path, None).await?;
    Ok(res.json())
}

async fn post_status(
    url: &str,
    access_token: String,
    status: &str,
    media_id: Option<Vec<String>>,
) -> Result<entities::Status, error::Error> {
    let client = generator(
        megalodon::SNS::Pleroma,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client
        .post_status(
            status.to_string(),
            Some(&PostStatusInputOptions {
                media_ids: media_id,
                poll: None,
                in_reply_to_id: None,
                sensitive: Some(true),
                spoiler_text: None,
                visibility: Some(StatusVisibility::Unlisted),
                scheduled_at: None,
                language: Some("en".to_string()),
                quote_id: None,
            }),
        )
        .await?;
    Ok(res.json())
}
