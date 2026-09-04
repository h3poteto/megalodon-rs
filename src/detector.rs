use serde::Deserialize;

use crate::{error, HttpClient, SNS};

#[derive(Deserialize, Debug)]
struct Links {
    links: Vec<Link>,
}

#[derive(Deserialize, Debug)]
struct Link {
    href: reqwest::Url,
    rel: String,
}

const NODEINFO_10: &str = "http://nodeinfo.diaspora.software/ns/schema/1.0";
const NODEINFO_20: &str = "http://nodeinfo.diaspora.software/ns/schema/2.0";
const NODEINFO_21: &str = "http://nodeinfo.diaspora.software/ns/schema/2.1";

#[derive(Deserialize, Debug)]
struct Nodeinfo10 {
    software: Software,
    metadata: Metadata,
}

#[derive(Deserialize, Debug)]
struct Nodeinfo20 {
    software: Software,
    metadata: Metadata,
}

#[derive(Deserialize, Debug)]
struct Nodeinfo21 {
    software: Software,
    metadata: Metadata,
}

#[derive(Deserialize, Debug)]
struct Software {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    upstream: Option<Upstream>,
}

#[derive(Deserialize, Debug)]
struct Upstream {
    name: String,
}

/// Detect which SNS the provided URL is. To detect SNS, the URL has to open `/api/v1/instance` or `/api/meta` endpoint.
pub async fn detector(client: &dyn HttpClient, url: &str) -> Result<SNS, error::Error> {
    let links = client
        .request(reqwest::Request::new(
            reqwest::Method::GET,
            format!("{}{}", url, "/.well-known/nodeinfo").parse()?,
        ))
        .await?
        .error_for_status()?
        .json::<Links>()
        .await?;

    let Some(link) = links
        .links
        .iter()
        .find(|l| l.rel == NODEINFO_20 || l.rel == NODEINFO_21 || l.rel == NODEINFO_10)
    else {
        return Err(error::Error::new_own(
            String::from("Could not find nodeinfo"),
            error::Kind::NodeinfoError,
            None,
            None,
            None,
        ));
    };

    match link.rel.as_str() {
        NODEINFO_10 => {
            let nodeinfo = client
                .request(reqwest::Request::new(
                    reqwest::Method::GET,
                    link.href.clone(),
                ))
                .await?
                .json::<Nodeinfo10>()
                .await?;
            match nodeinfo.software.name.as_str() {
                "akkoma" => Ok(SNS::Pleroma),
                "firefish" => Ok(SNS::Firefish),
                "friendica" => Ok(SNS::Friendica),
                "gotosocial" => Ok(SNS::Gotosocial),
                "hometown" => Ok(SNS::Mastodon),
                "iceshrimp" => Ok(SNS::Firefish),
                "mastodon" => Ok(SNS::Mastodon),
                "pleroma" => Ok(SNS::Pleroma),
                "pixelfed" => Ok(SNS::Pixelfed),
                _ => {
                    if let Some(upstream) = nodeinfo.metadata.upstream {
                        if upstream.name.to_lowercase() == "mastodon" {
                            return Ok(SNS::Mastodon);
                        }
                    }
                    Err(error::Error::new_own(
                        String::from("Unknown SNS"),
                        error::Kind::UnknownSNSError,
                        Some(url.to_string()),
                        None,
                        None,
                    ))
                }
            }
        }
        NODEINFO_20 => {
            let nodeinfo = client
                .request(reqwest::Request::new(
                    reqwest::Method::GET,
                    link.href.clone(),
                ))
                .await?
                .json::<Nodeinfo20>()
                .await?;
            match nodeinfo.software.name.as_str() {
                "akkoma" => Ok(SNS::Pleroma),
                "firefish" => Ok(SNS::Firefish),
                "friendica" => Ok(SNS::Friendica),
                "gotosocial" => Ok(SNS::Gotosocial),
                "hometown" => Ok(SNS::Mastodon),
                "iceshrimp" => Ok(SNS::Firefish),
                "mastodon" => Ok(SNS::Mastodon),
                "pleroma" => Ok(SNS::Pleroma),
                "pixelfed" => Ok(SNS::Pixelfed),
                _ => {
                    if let Some(upstream) = nodeinfo.metadata.upstream {
                        if upstream.name.to_lowercase() == "mastodon" {
                            return Ok(SNS::Mastodon);
                        }
                    }
                    Err(error::Error::new_own(
                        String::from("Unknown SNS"),
                        error::Kind::UnknownSNSError,
                        Some(url.to_string()),
                        None,
                        None,
                    ))
                }
            }
        }
        NODEINFO_21 => {
            let nodeinfo = client
                .request(reqwest::Request::new(
                    reqwest::Method::GET,
                    link.href.clone(),
                ))
                .await?
                .json::<Nodeinfo21>()
                .await?;
            match nodeinfo.software.name.as_str() {
                "akkoma" => Ok(SNS::Pleroma),
                "firefish" => Ok(SNS::Firefish),
                "friendica" => Ok(SNS::Friendica),
                "gotosocial" => Ok(SNS::Gotosocial),
                "hometown" => Ok(SNS::Mastodon),
                "iceshrimp" => Ok(SNS::Firefish),
                "mastodon" => Ok(SNS::Mastodon),
                "pleroma" => Ok(SNS::Pleroma),
                "pixelfed" => Ok(SNS::Pixelfed),
                _ => {
                    if let Some(upstream) = nodeinfo.metadata.upstream {
                        if upstream.name.to_lowercase() == "mastodon" {
                            return Ok(SNS::Mastodon);
                        }
                    }
                    Err(error::Error::new_own(
                        String::from("Unknown SNS"),
                        error::Kind::UnknownSNSError,
                        Some(url.to_string()),
                        None,
                        None,
                    ))
                }
            }
        }
        _ => Err(error::Error::new_own(
            String::from("Cound not find nodeinfo"),
            error::Kind::NodeinfoError,
            Some(url.to_string()),
            None,
            None,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detector_mastodon() {
        let sns = detector(&reqwest::Client::new(), "https://mastodon.social").await;

        assert!(sns.is_ok(), "{sns:?}");
        assert_eq!(sns.unwrap(), SNS::Mastodon);
    }

    #[tokio::test]
    async fn test_detector_pleroma() {
        let sns = detector(&reqwest::Client::new(), "https://pleroma.io").await;

        assert!(sns.is_ok(), "{sns:?}");
        assert_eq!(sns.unwrap(), SNS::Pleroma);
    }

    #[tokio::test]
    async fn test_detector_fedibird() {
        let sns = detector(&reqwest::Client::new(), "https://fedibird.com").await;

        assert!(sns.is_ok(), "{sns:?}");
        assert_eq!(sns.unwrap(), SNS::Mastodon);
    }

    #[tokio::test]
    async fn test_detector_friendica() {
        let sns = detector(&reqwest::Client::new(), "https://squeet.me").await;

        assert!(sns.is_ok(), "{sns:?}");
        assert_eq!(sns.unwrap(), SNS::Friendica);
    }

    #[tokio::test]
    async fn test_detector_akkoma() {
        let sns = detector(&reqwest::Client::new(), "https://blob.cat").await;

        assert!(sns.is_ok(), "{sns:?}");
        assert_eq!(sns.unwrap(), SNS::Pleroma);
    }

    #[tokio::test]
    async fn test_detector_firefish() {
        let sns = detector(&reqwest::Client::new(), "https://cybre.club").await;

        assert!(sns.is_ok(), "{sns:?}");
        assert_eq!(sns.unwrap(), SNS::Firefish);
    }

    #[tokio::test]
    async fn test_detector_gotosocial() {
        let sns = detector(
            // https://goblin.technology returns a 418 code for the default reqwest useragent
            &reqwest::Client::builder()
                .user_agent("megalodon")
                .build()
                .unwrap(),
            "https://goblin.technology",
        )
        .await;

        assert!(sns.is_ok(), "{sns:?}");
        assert_eq!(sns.unwrap(), SNS::Gotosocial);
    }

    #[tokio::test]
    async fn test_detector_kmyblue() {
        let sns = detector(&reqwest::Client::new(), "https://kmy.blue").await;

        assert!(sns.is_ok(), "{sns:?}");
        assert_eq!(sns.unwrap(), SNS::Mastodon);
    }

    #[tokio::test]
    async fn test_detector_pixelfed() {
        let sns = detector(&reqwest::Client::new(), "https://pixelfed.social").await;

        assert!(sns.is_ok(), "{sns:?}");
        assert_eq!(sns.unwrap(), SNS::Pixelfed);
    }
}
