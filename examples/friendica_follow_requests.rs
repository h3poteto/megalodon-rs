use std::env;

use megalodon::{error, generator, megalodon::FollowRequest};

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("FRIENDICA_URL") else {
        println!("Specify FRIENDICA_URL!!");
        return
    };
    let Ok(token) = env::var("FRIENDICA_ACCESS_TOKEN") else {
        println!("Specify FRIENDICA_ACCESS_TOKEN!!");
        return
    };

    let res = follow_requests(url.as_str(), token).await;
    match res {
        Ok(res) => res
            .into_iter()
            .map(|f| match f {
                FollowRequest::FollowRequest(req) => println!("FollowRequest: {:#?}", req),
                FollowRequest::Account(acct) => println!("Account: {:#?}", acct),
            })
            .collect(),
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}

async fn follow_requests(
    url: &str,
    access_token: String,
) -> Result<Vec<FollowRequest>, error::Error> {
    let client = generator(
        megalodon::SNS::Friendica,
        url.to_string(),
        Some(access_token),
        None,
    );
    let res = client.get_follow_requests(None).await?;

    Ok(res.json())
}
