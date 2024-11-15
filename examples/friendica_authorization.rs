use megalodon::generator;
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("FRIENDICA_URL") else {
        println!("Specify FRIENDICA_URL!!");
        return;
    };

    let client = generator(megalodon::SNS::Friendica, url, None, None).unwrap();
    let options = megalodon::megalodon::AppInputOptions {
        scopes: Some(
            [
                String::from("read"),
                String::from("write"),
                String::from("follow"),
            ]
            .to_vec(),
        ),
        ..Default::default()
    };

    match client
        .register_app(String::from("TestMegalodon"), &options)
        .await
    {
        Ok(app_data) => {
            let client_id = app_data.client_id;
            let client_secret = app_data.client_secret;
            println!("Authorization URL is generated");
            println!("{}", app_data.url.unwrap());

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
