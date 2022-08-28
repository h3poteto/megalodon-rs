use megalodon::{mastodon::Mastodon, Megalodon};

#[tokio::main]
async fn main() {
    let client = Mastodon::new("https://fedibird.com".to_string(), None, None);
    let res = client.get_instance();
    match res.await {
        Ok(response) => {
            let instance = response.json();
            println!("{:#?}", instance);
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }
}
