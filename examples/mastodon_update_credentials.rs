use megalodon::{generator, SNS};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("MASTODON_URL") else {
        println!("Specify MASTODON_URL!!");
        return
    };
    let Ok(token) = env::var("MASTODON_ACCESS_TOKEN") else {
        println!("Specify MASTODON_ACCESS_TOKEN!!");
        return
    };

    let client = generator(SNS::Mastodon, url, Some(token), None);

    let update_creds = megalodon::megalodon::UpdateCredentialsInputOptions {
        fields_attributes: Some(vec![megalodon::megalodon::CredentialsFieldAttribute {
            name: "Test".to_string(),
            value: "test".to_string(),
        }]),
        ..Default::default()
    };

    let res = client
        .update_credentials(Some(&update_creds))
        .await
        .unwrap();

    println!("{:#?}", res.json())
}
