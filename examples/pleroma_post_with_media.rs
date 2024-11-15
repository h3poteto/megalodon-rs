use megalodon::{
    entities::{self, StatusVisibility},
    error, generator,
    megalodon::PostStatusInputOptions,
};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("PLEROMA_URL") else {
        println!("Specify PLEROMA_URL!!");
        return;
    };
    let Ok(token) = env::var("PLEROMA_ACCESS_TOKEN") else {
        println!("Specify PLEROMA_ACCESS_TOKEN!!");
        return;
    };

    let file_path = "./sample.jpg".to_string();
    let Ok(res) = upload_media(url.as_str(), token.to_owned(), file_path.to_string()).await else {
        println!("failed to upload media");
        return;
    };
    match res {
        entities::UploadMedia::AsyncAttachment(_) => {
            println!("unexpected async upload");
        }
        entities::UploadMedia::Attachment(media) => {
            let media_id_1 = media.id;
            let file_path = "./sample2.jpg".to_string();
            let Ok(res) = upload_media(url.as_str(), token.to_owned(), file_path).await else {
                println!("failed to upload media");
                return;
            };
            match res {
                entities::UploadMedia::AsyncAttachment(_) => {
                    println!("unexpected async upload");
                }
                entities::UploadMedia::Attachment(media) => {
                    let media_id_2 = media.id;
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
            }
        }
    }
}

async fn upload_media(
    url: &str,
    access_token: String,
    file_path: String,
) -> Result<entities::UploadMedia, error::Error> {
    let client = generator(
        megalodon::SNS::Pleroma,
        url.to_string(),
        Some(access_token),
        None,
    )?;
    let res = client.upload_media(file_path, None).await?;
    Ok(res.json())
}

async fn post_status(
    url: &str,
    access_token: String,
    status: &str,
    media_id: Option<Vec<String>>,
) -> Result<megalodon::megalodon::PostStatusOutput, error::Error> {
    let client = generator(
        megalodon::SNS::Pleroma,
        url.to_string(),
        Some(access_token),
        None,
    )?;
    let res = client
        .post_status(
            status.to_string(),
            Some(&PostStatusInputOptions {
                media_ids: media_id,
                sensitive: Some(true),
                visibility: Some(StatusVisibility::Unlisted),
                language: Some("en".to_string()),
                ..Default::default()
            }),
        )
        .await?;
    Ok(res.json())
}
