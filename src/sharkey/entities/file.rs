use crate::entities as MegalodonEntities;

use regex::Regex;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    id: String,
    // created_at: DateTime<Utc>,
    // name: String,
    r#type: String,
    // md5: String,
    // size: u32,
    pub is_sensitive: bool,
    properties: Properties,
    url: Option<String>,
    thumbnail_url: Option<String>,
    blurhash: Option<String>,
    comment: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    width: Option<u32>,
    height: Option<u32>,
    // orientation: u32,
    // avg_color: String,
}

impl From<File> for MegalodonEntities::Attachment {
    fn from(val: File) -> Self {
        let mut url = "".to_string();
        if let Some(u) = val.url.clone() {
            url = u;
        };

        let mut attachment_type = MegalodonEntities::attachment::AttachmentType::Unknown;
        if val.r#type.as_str() == "image/gif" {
            attachment_type = MegalodonEntities::attachment::AttachmentType::Gifv;
        } else if Regex::new(r"^image").unwrap().is_match(val.r#type.as_str()) {
            attachment_type = MegalodonEntities::attachment::AttachmentType::Image;
        }
        if Regex::new(r"^audio").unwrap().is_match(val.r#type.as_str()) {
            attachment_type = MegalodonEntities::attachment::AttachmentType::Audio;
        }
        if Regex::new(r"^video").unwrap().is_match(val.r#type.as_str()) {
            attachment_type = MegalodonEntities::attachment::AttachmentType::Video;
        }

        let meta = MegalodonEntities::attachment::AttachmentMeta {
            original: Some(MegalodonEntities::attachment::MetaSub {
                width: val.properties.width,
                height: val.properties.height,
                size: None,
                aspect: None,
                frame_rate: None,
                duration: None,
                bitrate: None,
            }),
            small: None,
            focus: None,
            length: None,
            duration: None,
            fps: None,
            size: None,
            width: val.properties.width,
            height: val.properties.height,
            aspect: None,
            audio_encode: None,
            audio_bitrate: None,
            audio_channel: None,
        };

        MegalodonEntities::Attachment {
            id: val.id,
            r#type: attachment_type,
            url,
            remote_url: val.url.clone(),
            preview_url: val.thumbnail_url,
            text_url: val.url,
            meta: Some(meta),
            description: val.comment,
            blurhash: val.blurhash,
        }
    }
}

impl From<File> for MegalodonEntities::UploadMedia {
    fn from(val: File) -> Self {
        MegalodonEntities::UploadMedia::Attachment(val.into())
    }
}
