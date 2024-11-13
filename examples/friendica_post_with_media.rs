use megalodon::{
    entities::{self, StatusVisibility},
    error, generator,
    megalodon::PostStatusInputOptions,
};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("FRIENDICA_URL") else {
        println!("Specify FRIENDICA_URL!!");
        return;
    };
    let Ok(token) = env::var("FRIENDICA_ACCESS_TOKEN") else {
        println!("Specify FRIENDICA_ACCESS_TOKEN!!");
        return;
    };

    let client = generator(megalodon::SNS::Friendica, url, Some(token), None).unwrap();

    let file_path = "./sample.jpg".to_string();
    let Ok(res) = upload_media(&client, file_path.to_string()).await else {
        println!("failed to upload media");
        return;
    };

    let media_1: entities::Attachment;
    match res {
        entities::UploadMedia::AsyncAttachment(m) => {
            match wait_until_uploaded(&client, &m.id).await {
                Ok(res) => media_1 = res,
                Err(err) => {
                    println!("{:#?}", err);
                    return;
                }
            }
        }
        entities::UploadMedia::Attachment(m) => {
            media_1 = m;
        }
    }
    let media_id_1 = media_1.id;
    let file_path = "./sample2.jpg".to_string();
    let Ok(res) = upload_media(&client, file_path).await else {
        println!("error");
        return;
    };
    let media_2: entities::Attachment;
    match res {
        entities::UploadMedia::AsyncAttachment(m) => {
            match wait_until_uploaded(&client, &m.id).await {
                Ok(res) => media_2 = res,
                Err(err) => {
                    println!("{:#?}", err);
                    return;
                }
            }
        }
        entities::UploadMedia::Attachment(m) => {
            media_2 = m;
        }
    }
    let media_id_2 = media_2.id;
    let media_ids = vec![media_id_1, media_id_2];
    match post_status(&client, "Post with attached media", Some(media_ids)).await {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn wait_until_uploaded(
    client: &Box<dyn megalodon::Megalodon + Send + Sync>,
    id: &str,
) -> Result<entities::Attachment, error::Error> {
    loop {
        let res = client.get_media(id.to_string()).await;
        return match res {
            Ok(res) => Ok(res.json()),
            Err(err) => match err {
                error::Error::OwnError(ref own_err) => match own_err.kind {
                    error::Kind::HTTPPartialContentError => continue,
                    _ => Err(err),
                },
                _ => Err(err),
            },
        };
    }
}

async fn upload_media(
    client: &Box<dyn megalodon::Megalodon + Send + Sync>,
    file_path: String,
) -> Result<entities::UploadMedia, error::Error> {
    let res = client.upload_media(file_path, None).await?;
    Ok(res.json())
}

async fn post_status(
    client: &Box<dyn megalodon::Megalodon + Send + Sync>,
    status: &str,
    media_id: Option<Vec<String>>,
) -> Result<megalodon::megalodon::PostStatusOutput, error::Error> {
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
