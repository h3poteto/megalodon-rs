use std::env;

use megalodon::{
    entities, error, generator,
    megalodon::{SearchInputOptions, SearchType},
};

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("MASTODON_URL") else {
        println!("Specify MASTODON_URL!!");
        return;
    };
    let Ok(token) = env::var("MASTODON_ACCESS_TOKEN") else {
        println!("Specify MASTODON_ACCESS_TOKEN!!");
        return;
    };

    let res = search(url.as_str(), token, "h3poteto").await;
    match res {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn search(
    url: &str,
    access_token: String,
    query: &str,
) -> Result<entities::Results, error::Error> {
    let client = generator(
        megalodon::SNS::Mastodon,
        url.to_string(),
        Some(access_token),
        None,
    );
    let options = SearchInputOptions {
        r#type: Some(SearchType::Accounts),
        limit: None,
        max_id: None,
        min_id: None,
        resolve: Some(true),
        offset: None,
        following: None,
        account_id: None,
        exclude_unreviewed: None,
    };
    let res = client.search(query.to_string(), Some(&options)).await?;
    Ok(res.json())
}
