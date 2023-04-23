use crate::entities as MegalodonEntities;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Attachment {
    id: String,
    r#type: AttachmentType,
    url: Option<String>,
    remote_url: Option<String>,
    preview_url: Option<String>,
    text_url: Option<String>,
    meta: Option<AttachmentMeta>,
    description: Option<String>,
    blurhash: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AttachmentMeta {
    original: Option<MetaSub>,
    small: Option<MetaSub>,
    focus: Option<Focus>,
    length: Option<String>,
    duration: Option<f64>,
    fps: Option<u32>,
    size: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    aspect: Option<f64>,
    audio_encode: Option<String>,
    audio_bitrate: Option<String>,
    audio_channel: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetaSub {
    // For Image, Gifv, Video
    width: Option<u32>,
    height: Option<u32>,
    size: Option<String>,
    aspect: Option<f64>,

    // For Gifv, Video
    frame_rate: Option<String>,

    // For Audio, Gifv, Video
    duration: Option<f64>,
    bitrate: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Focus {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AttachmentType {
    Image,
    Gifv,
    Video,
    Audio,
    Unknown,
}

impl Into<MegalodonEntities::attachment::AttachmentType> for AttachmentType {
    fn into(self) -> MegalodonEntities::attachment::AttachmentType {
        match self {
            AttachmentType::Image => MegalodonEntities::attachment::AttachmentType::Image,
            AttachmentType::Gifv => MegalodonEntities::attachment::AttachmentType::Gifv,
            AttachmentType::Video => MegalodonEntities::attachment::AttachmentType::Video,
            AttachmentType::Audio => MegalodonEntities::attachment::AttachmentType::Audio,
            AttachmentType::Unknown => MegalodonEntities::attachment::AttachmentType::Unknown,
        }
    }
}

impl Into<MegalodonEntities::Attachment> for Attachment {
    fn into(self) -> MegalodonEntities::Attachment {
        MegalodonEntities::Attachment {
            id: self.id,
            r#type: self.r#type.into(),
            url: self.url.unwrap(),
            remote_url: self.remote_url,
            preview_url: self.preview_url,
            text_url: self.text_url,
            meta: self.meta.map(|i| i.into()),
            description: self.description,
            blurhash: self.blurhash,
        }
    }
}

impl Into<MegalodonEntities::UploadMedia> for Attachment {
    fn into(self) -> MegalodonEntities::UploadMedia {
        if let Some(url) = self.url {
            MegalodonEntities::UploadMedia::Attachment(MegalodonEntities::Attachment {
                id: self.id,
                r#type: self.r#type.into(),
                url,
                remote_url: self.remote_url,
                preview_url: self.preview_url,
                text_url: self.text_url,
                meta: self.meta.map(|i| i.into()),
                description: self.description,
                blurhash: self.blurhash,
            })
        } else {
            MegalodonEntities::UploadMedia::AsyncAttachment(MegalodonEntities::AsyncAttachment {
                id: self.id,
                r#type: self.r#type.into(),
                url: None,
                remote_url: self.remote_url,
                preview_url: self.preview_url,
                text_url: self.text_url,
                meta: self.meta.map(|i| i.into()),
                description: self.description,
                blurhash: self.blurhash,
            })
        }
    }
}

impl Into<MegalodonEntities::attachment::AttachmentMeta> for AttachmentMeta {
    fn into(self) -> MegalodonEntities::attachment::AttachmentMeta {
        MegalodonEntities::attachment::AttachmentMeta {
            original: self.original.map(|i| i.into()),
            small: self.small.map(|i| i.into()),
            focus: self.focus.map(|i| i.into()),
            length: self.length,
            duration: self.duration,
            fps: self.fps,
            size: self.size,
            width: self.width,
            height: self.height,
            aspect: self.aspect,
            audio_encode: self.audio_encode,
            audio_bitrate: self.audio_bitrate,
            audio_channel: self.audio_channel,
        }
    }
}

impl Into<MegalodonEntities::attachment::MetaSub> for MetaSub {
    fn into(self) -> MegalodonEntities::attachment::MetaSub {
        MegalodonEntities::attachment::MetaSub {
            width: self.width,
            height: self.height,
            size: self.size,
            aspect: self.aspect,
            frame_rate: self.frame_rate,
            duration: self.duration,
            bitrate: self.bitrate,
        }
    }
}

impl Into<MegalodonEntities::attachment::Focus> for Focus {
    fn into(self) -> MegalodonEntities::attachment::Focus {
        MegalodonEntities::attachment::Focus {
            x: self.x,
            y: self.y,
        }
    }
}
