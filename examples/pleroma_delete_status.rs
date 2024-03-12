use std::env;

use megalodon::generator;

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
    let client = generator(megalodon::SNS::Pleroma, url.to_string(), Some(token), None);
    println!("Target status_id: ");
    let mut status_id = String::new();
    std::io::stdin().read_line(&mut status_id).ok();

    match client.delete_status(status_id).await {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err)
        }
    }
}
