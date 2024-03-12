use megalodon::generator;
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let Ok(url) = env::var("FIREFISH_URL") else {
        println!("Specify FIREFISH_URL!!");
        return;
    };

    let client = generator(megalodon::SNS::Firefish, url.to_string(), None, None);
    let options = megalodon::megalodon::AppInputOptions {
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
            println!("{}", app_data.session_token.clone().unwrap());
            println!("Press enter key after approve in the website: ");
            let mut code = String::new();
            std::io::stdin().read_line(&mut code).ok();

            match client
                .fetch_access_token(
                    client_id,
                    client_secret,
                    app_data.session_token.unwrap(),
                    megalodon::default::NO_REDIRECT.to_string(),
                )
                .await
            {
                Ok(token_data) => {
                    println!("access_token: {}", token_data.access_token);
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
