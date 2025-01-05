use megalodon::{entities, error, generator};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("PIXELFED_URL") else {
        println!("Specify PIXELFED_URL!!");
        return;
    };
    let Ok(token) = env::var("PIXELFED_ACCESS_TOKEN") else {
        println!("Specify PIXELFED_ACCESS_TOKEN!!");
        return;
    };

    let res = get_notifications(url.as_str(), token).await;
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn get_notifications(
    url: &str,
    access_token: String,
) -> Result<Vec<entities::Notification>, error::Error> {
    let client = generator(
        megalodon::SNS::Pixelfed,
        url.to_string(),
        Some(access_token),
        None,
    )?;
    let res = client.get_notifications(None).await?;
    Ok(res.json())
}
