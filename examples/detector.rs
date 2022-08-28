use megalodon::detector;

#[tokio::main]
async fn main() {
    let sns = detector("https://fedibird.com").await;
    println!("{:#?}", sns);
}
