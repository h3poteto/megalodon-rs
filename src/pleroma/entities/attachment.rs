use crate::entities as MegalodonEntities;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Attachment {
    id: String,
    r#type: AttachmentType,
    url: String,
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

impl From<AttachmentType> for MegalodonEntities::attachment::AttachmentType {
    fn from(val: AttachmentType) -> Self {
        match val {
            AttachmentType::Image => MegalodonEntities::attachment::AttachmentType::Image,
            AttachmentType::Gifv => MegalodonEntities::attachment::AttachmentType::Gifv,
            AttachmentType::Video => MegalodonEntities::attachment::AttachmentType::Video,
            AttachmentType::Audio => MegalodonEntities::attachment::AttachmentType::Audio,
            AttachmentType::Unknown => MegalodonEntities::attachment::AttachmentType::Unknown,
        }
    }
}

impl From<Attachment> for MegalodonEntities::Attachment {
    fn from(val: Attachment) -> Self {
        MegalodonEntities::Attachment {
            id: val.id,
            r#type: val.r#type.into(),
            url: val.url,
            remote_url: val.remote_url,
            preview_url: val.preview_url,
            text_url: val.text_url,
            meta: val.meta.map(|i| i.into()),
            description: val.description,
            blurhash: val.blurhash,
        }
    }
}

impl From<AttachmentMeta> for MegalodonEntities::attachment::AttachmentMeta {
    fn from(val: AttachmentMeta) -> Self {
        MegalodonEntities::attachment::AttachmentMeta {
            original: val.original.map(|i| i.into()),
            small: val.small.map(|i| i.into()),
            focus: val.focus.map(|i| i.into()),
            length: val.length,
            duration: val.duration,
            fps: val.fps,
            size: val.size,
            width: val.width,
            height: val.height,
            aspect: val.aspect,
            audio_encode: val.audio_encode,
            audio_bitrate: val.audio_bitrate,
            audio_channel: val.audio_channel,
        }
    }
}

impl From<MetaSub> for MegalodonEntities::attachment::MetaSub {
    fn from(val: MetaSub) -> Self {
        MegalodonEntities::attachment::MetaSub {
            width: val.width,
            height: val.height,
            size: val.size,
            aspect: val.aspect,
            frame_rate: val.frame_rate,
            duration: val.duration,
            bitrate: val.bitrate,
        }
    }
}

impl From<Focus> for MegalodonEntities::attachment::Focus {
    fn from(val: Focus) -> Self {
        MegalodonEntities::attachment::Focus { x: val.x, y: val.y }
    }
}
