use std::env;

use chrono::{DateTime, Duration, Utc};
use megalodon::{entities, error, generator, megalodon::PostStatusInputOptions};

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("PLEROMA_URL") else {
        println!("Specify PLEROMA_URL!!");
        return;
    };
    let Ok(token) = env::var("PLEROMA_ACCESS_TOKEN") else {
        println!("Specify PLEROMA_ACCESS_TOKEN!!");
        return;
    };

    let client = generator(megalodon::SNS::Pleroma, url, Some(token), None);

    let scheduled_at = Utc::now() + Duration::try_minutes(6).unwrap();
    println!("scheduled at {:#?}", scheduled_at);

    match post_status(&client, "Test", scheduled_at).await {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn post_status(
    client: &Box<dyn megalodon::Megalodon + Send + Sync>,
    status: &str,
    scheduled_at: DateTime<Utc>,
) -> Result<megalodon::megalodon::PostStatusOutput, error::Error> {
    let res = client
        .post_status(
            status.to_string(),
            Some(&PostStatusInputOptions {
                visibility: Some(entities::status::StatusVisibility::Private),
                scheduled_at: Some(scheduled_at),
                ..Default::default()
            }),
        )
        .await?;

    Ok(res.json())
}
