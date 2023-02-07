use megalodon::generator;
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let Ok(url) = env::var("MASTODON_URL") else {
        println!("Specify MASTODON_URL!!");
        return
    };
    let Ok(client_id) = env::var("MASTODON_CLIENT_ID") else {
        println!("Specify MASTODON_CLIENT_ID");
        return
    };
    let Ok(client_secret) = env::var("MASTODON_CLIENT_SECRET") else {
        println!("Specify MASTODON_CLIENT_SECRET");
        return
    };
    let Ok(redirect) = env::var("MASTODON_REDIRECT_URL") else {
        println!("Specify MASTODON_REDIRECT_URL");
        return
    };
    let client = generator(megalodon::SNS::Mastodon, url, None, None);

    let scopes = "read read:accounts read:bookmarks read:favourites read:statuses write write:bookmarks write:favourites write:media write:statuses follow".split(" ")
    .map(|e| e.to_string())
    .collect();

    match client
        .authorize_user_code_url(
            client_id.clone(),
            client_secret.clone(),
            scopes,
            redirect.clone(),
        )
        .await
    {
        Ok(url) => {
            println!("Authorization URL is generated");
            println!("{url}");

            println!("Enter authorization code from website: ");

            let mut code = String::new();
            std::io::stdin().read_line(&mut code).ok();

            match client
                .fetch_access_token(
                    client_id,
                    client_secret,
                    code.trim().to_string(),
                    megalodon::default::NO_REDIRECT.to_string(),
                )
                .await
            {
                Ok(token_data) => {
                    println!("access_token: {}", token_data.access_token);
                    if let Some(refresh) = token_data.refresh_token {
                        println!("refresh_token: {}", refresh);
                    }
                }
                Err(err) => {
                    println!("{:#?}", err);
                }
            }
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}
