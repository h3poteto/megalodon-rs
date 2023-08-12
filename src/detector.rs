use serde::Deserialize;

use crate::{error, SNS};

#[derive(Deserialize, Debug)]
struct Links {
    links: Vec<Link>,
}

#[derive(Deserialize, Debug)]
struct Link {
    href: String,
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
pub async fn detector(url: &str) -> Result<SNS, error::Error> {
    let client = reqwest::Client::builder().user_agent("megalodon").build()?;
    let links = client
        .get(format!("{}{}", url, "/.well-known/nodeinfo"))
        .send()
        .await?
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
        ));
    };

    match link.rel.as_str() {
        NODEINFO_10 => {
            let nodeinfo = client
                .get(link.href.as_str())
                .send()
                .await?
                .json::<Nodeinfo10>()
                .await?;
            match nodeinfo.software.name.as_str() {
                "pleroma" => Ok(SNS::Pleroma),
                "akkoma" => Ok(SNS::Pleroma),
                "mastodon" => Ok(SNS::Mastodon),
                "friendica" => Ok(SNS::Friendica),
                "hometown" => Ok(SNS::Mastodon),
                "firefish" => Ok(SNS::Firefish),
                "gotosocial" => Ok(SNS::Gotosocial),
                _ => {
                    if let Some(upstream) = nodeinfo.metadata.upstream {
                        if upstream.name == "mastodon" {
                            return Ok(SNS::Mastodon);
                        }
                    }
                    Err(error::Error::new_own(
                        String::from("Unknown SNS"),
                        error::Kind::UnknownSNSError,
                        Some(url.to_string()),
                        None,
                    ))
                }
            }
        }
        NODEINFO_20 => {
            let nodeinfo = client
                .get(link.href.as_str())
                .send()
                .await?
                .json::<Nodeinfo20>()
                .await?;
            match nodeinfo.software.name.as_str() {
                "pleroma" => Ok(SNS::Pleroma),
                "akkoma" => Ok(SNS::Pleroma),
                "mastodon" => Ok(SNS::Mastodon),
                "friendica" => Ok(SNS::Friendica),
                "hometown" => Ok(SNS::Mastodon),
                "firefish" => Ok(SNS::Firefish),
                "gotosocial" => Ok(SNS::Gotosocial),
                _ => {
                    if let Some(upstream) = nodeinfo.metadata.upstream {
                        if upstream.name == "mastodon" {
                            return Ok(SNS::Mastodon);
                        }
                    }
                    Err(error::Error::new_own(
                        String::from("Unknown SNS"),
                        error::Kind::UnknownSNSError,
                        Some(url.to_string()),
                        None,
                    ))
                }
            }
        }
        NODEINFO_21 => {
            let nodeinfo = client
                .get(link.href.as_str())
                .send()
                .await?
                .json::<Nodeinfo21>()
                .await?;
            match nodeinfo.software.name.as_str() {
                "pleroma" => Ok(SNS::Pleroma),
                "akkoma" => Ok(SNS::Pleroma),
                "mastodon" => Ok(SNS::Mastodon),
                "friendica" => Ok(SNS::Friendica),
                "hometown" => Ok(SNS::Mastodon),
                "firefish" => Ok(SNS::Firefish),
                "gotosocial" => Ok(SNS::Gotosocial),
                _ => {
                    if let Some(upstream) = nodeinfo.metadata.upstream {
                        if upstream.name == "mastodon" {
                            return Ok(SNS::Mastodon);
                        }
                    }
                    Err(error::Error::new_own(
                        String::from("Unknown SNS"),
                        error::Kind::UnknownSNSError,
                        Some(url.to_string()),
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
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detector_mastodon() {
        let sns = detector("https://mastodon.social").await;

        assert!(sns.is_ok());
        assert_eq!(sns.unwrap(), SNS::Mastodon);
    }

    #[tokio::test]
    async fn test_detector_pleroma() {
        let sns = detector("https://pleroma.io").await;

        assert!(sns.is_ok());
        assert_eq!(sns.unwrap(), SNS::Pleroma);
    }

    #[tokio::test]
    async fn test_detector_fedibird() {
        let sns = detector("https://fedibird.com").await;

        assert!(sns.is_ok());
        assert_eq!(sns.unwrap(), SNS::Mastodon);
    }

    #[tokio::test]
    async fn test_detector_friendica() {
        let sns = detector("https://squeet.me").await;

        assert!(sns.is_ok());
        assert_eq!(sns.unwrap(), SNS::Friendica);
    }

    #[tokio::test]
    async fn test_detector_akkoma() {
        let sns = detector("https://blob.cat").await;

        assert!(sns.is_ok());
        assert_eq!(sns.unwrap(), SNS::Pleroma);
    }

    #[tokio::test]
    async fn test_detector_firefish() {
        let sns = detector("https://calckey.world").await;

        assert!(sns.is_ok());
        assert_eq!(sns.unwrap(), SNS::Firefish);
    }

    #[tokio::test]
    async fn test_detector_gotosocial() {
        let sns = detector("https://goblin.technology").await;

        assert!(sns.is_ok());
        assert_eq!(sns.unwrap(), SNS::Gotosocial);
    }
}
