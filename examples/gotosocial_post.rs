use megalodon::generator;
use std::{env, io};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("GOTOSOCIAL_URL") else {
        println!("Specify GOTOSOCIAL_URL!!");
        return;
    };
    let Ok(token) = env::var("GOTOSOCIAL_ACCESS_TOKEN") else {
        println!("Specify GOTOSOCIAL_ACCESS_TOKEN!!");
        return;
    };

    let client = generator(megalodon::SNS::Gotosocial, url, Some(token), None).unwrap();

    let res = post_status(&client).await;
    match res {
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
) -> Result<megalodon::megalodon::PostStatusOutput, megalodon::error::Error> {
    println!("Input text:");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("failed to read from stdin");
    let text = buffer.trim().to_string();
    let options = megalodon::megalodon::PostStatusInputOptions {
        visibility: Some(megalodon::entities::StatusVisibility::Public),
        ..Default::default()
    };
    let res = client.post_status(text, Some(&options)).await?;

    Ok(res.json())
}
