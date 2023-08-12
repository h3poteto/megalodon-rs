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

use std::{fmt, str::FromStr};

pub mod default;
pub mod detector;
pub mod entities;
pub mod error;
pub mod firefish;
pub mod friendica;
pub mod gotosocial;
pub mod mastodon;
pub mod megalodon;
pub mod oauth;
pub mod pleroma;
pub mod response;
pub mod streaming;

pub use self::megalodon::Megalodon;
pub use detector::detector;
use serde::{Deserialize, Serialize};
pub use streaming::Streaming;

/// Which SNS.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SNS {
    /// SNS is Mastodon.
    Mastodon,
    /// SNS is Pleroma.
    Pleroma,
    /// SNS is Friendica.
    Friendica,
    /// SNS is Firefish.
    Firefish,
}

impl fmt::Display for SNS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SNS::Mastodon => write!(f, "mastodon"),
            SNS::Pleroma => write!(f, "pleroma"),
            SNS::Friendica => write!(f, "friendica"),
            SNS::Firefish => write!(f, "firefish"),
        }
    }
}

impl FromStr for SNS {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mastodon" => Ok(SNS::Mastodon),
            "pleroma" => Ok(SNS::Pleroma),
            "friendica" => Ok(SNS::Friendica),
            "firefish" => Ok(SNS::Firefish),
            &_ => Err(format!("Unknown sns: {}", s)),
        }
    }
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
        SNS::Friendica => {
            let friendica = friendica::Friendica::new(base_url, access_token, user_agent);
            Box::new(friendica)
        }
        SNS::Mastodon => {
            let mastodon = mastodon::Mastodon::new(base_url, access_token, user_agent);
            Box::new(mastodon)
        }
        SNS::Firefish => {
            let firefish = firefish::Firefish::new(base_url, access_token, user_agent);
            Box::new(firefish)
        }
    }
}
