use std::env;

use megalodon::{
    entities, error, generator,
    megalodon::{SearchInputOptions, SearchType},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("SHARKEY_URL") else {
        println!("Specify SHARKEY_URL!!");
        return;
    };
    let Ok(token) = env::var("SHARKEY_ACCESS_TOKEN") else {
        println!("Specify SHARKEY_ACCESS_TOKEN!!");
        return;
    };

    let res = search(url.as_str(), token, "lemonlolita").await;
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
        megalodon::SNS::Sharkey,
        url.to_string(),
        Some(access_token),
        None,
    )?;
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
