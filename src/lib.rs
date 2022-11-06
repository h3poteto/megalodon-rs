#![deny(missing_debug_implementations)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # Megalodon
//! The `megalodon` is a client library for Mastodon and Pleroma. It provides REST API and streaming method which uses WebSocket. By using this library, you can take Mastodon and Pleroma with the same interface.
//!
//! ## Making Mastodon request
//! For a request without authentication.
//!
//! ```rust
//! # use megalodon;
//! # use megalodon::error::Error;
//! #
//! # async fn run() -> Result<(), Error> {
//! let client = megalodon::generator(
//!   megalodon::SNS::Mastodon,
//!   String::from("https://fedibird.com"),
//!   None,
//!   None,
//! );
//! let res = client.get_instance().await?;
//! println!("{:#?}", res.json());
//! # Ok(())
//! # }
//! ```
//!
//! ## Making Mastodon request with authentication
//! For a request with authentication.
//!
//! ```rust
//! # use megalodon;
//! # use megalodon::error::Error;
//! #
//! # async fn run() -> Result<(), Error> {
//! let client = megalodon::generator(
//!   megalodon::SNS::Mastodon,
//!   String::from("https://fedibird.com"),
//!   Some(String::from("your access token")),
//!   None,
//! );
//! let res = client.verify_account_credentials().await?;
//! println!("{:#?}", res.json());
//! # Ok(())
//! # }
//! ```

use serde::Deserialize;

pub mod default;
pub mod entities;
pub mod error;
pub mod mastodon;
pub mod megalodon;
pub mod oauth;
pub mod pleroma;
pub mod response;
pub mod streaming;

pub use self::megalodon::Megalodon;
pub use streaming::Streaming;

#[derive(Deserialize)]
struct Instance {
    title: String,
    uri: String,
    urls: entities::URLs,
    version: String,
}

/// Detect which SNS the provided URL is. To detect SNS, the URL has to open `/api/v1/instance` or `/api/meta` endpoint.
pub async fn detector(url: &str) -> Result<SNS, error::Error> {
    let res = reqwest::get(format!("{}{}", url, "/api/v1/instance")).await;

    match res {
        Ok(res) => {
            let obj = res.json::<Instance>().await;
            match obj {
                Ok(json) => {
                    if json.version.contains("Pleroma") == true {
                        Ok(SNS::Pleroma)
                    } else {
                        Ok(SNS::Mastodon)
                    }
                }
                Err(err) => Err(err.into()),
            }
        }
        Err(_) => {
            let client = reqwest::Client::new();
            let res = client.post(format!("{}{}", url, "/api/meta")).send().await;
            match res {
                Ok(_) => Ok(SNS::Misskey),
                Err(err) => Err(err.into()),
            }
        }
    }
}

/// Which SNS.
#[derive(Debug, Clone)]
pub enum SNS {
    /// SNS is Mastodon.
    Mastodon,
    /// SNS is Pleroma.
    Pleroma,
    /// SNS is Misskey.
    Misskey,
}

/// Generate an API client which satisfies megalodon trait.
pub fn generator(
    sns: SNS,
    base_url: String,
    access_token: Option<String>,
    user_agent: Option<String>,
) -> Box<dyn Megalodon + Send + Sync> {
    match sns {
        SNS::Pleroma => {
            let pleroma = pleroma::Pleroma::new(base_url, access_token, user_agent);
            Box::new(pleroma)
        }
        _ => {
            let mastodon = mastodon::Mastodon::new(base_url, access_token, user_agent);
            Box::new(mastodon)
        }
    }
}
