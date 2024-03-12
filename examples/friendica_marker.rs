use megalodon::{entities, error, generator};
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
    let Ok(notification_id) = env::var("NOTIFICATION_ID") else {
        println!("Specify NOTIFICATION_ID!!");
        return;
    };

    let save = save_marker(url.as_str(), token.clone(), notification_id).await;
    match save {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    let res = get_markers(url.as_str(), token).await;
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn save_marker(
    url: &str,
    access_token: String,
    id: String,
) -> Result<entities::Marker, error::Error> {
    let client = generator(
        megalodon::SNS::Friendica,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client
        .save_markers(Some(&megalodon::megalodon::SaveMarkersInputOptions {
            home: None,
            notifications: Some(megalodon::megalodon::Marker {
                last_reading_id: id,
            }),
        }))
        .await?;

    Ok(res.json())
}

async fn get_markers(url: &str, access_token: String) -> Result<entities::Marker, error::Error> {
    let client = generator(
        megalodon::SNS::Friendica,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client
        .get_markers(vec![String::from("home"), String::from("notifications")])
        .await?;
    Ok(res.json())
}
