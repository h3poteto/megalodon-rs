use core::fmt;
use serde::{de, ser, Deserialize};
use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Attachment {
    pub id: String,
    pub r#type: AttachmentType,
    pub url: String,
    pub remote_url: Option<String>,
    pub preview_url: String,
    pub text_url: Option<String>,
    pub meta: Option<AttachmentMeta>,
    pub description: Option<String>,
    pub blurhash: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AttachmentMeta {
    pub original: Option<MetaSub>,
    pub small: Option<MetaSub>,
    pub focus: Option<Focus>,
    pub length: Option<String>,
    pub duration: Option<f64>,
    pub fps: Option<u32>,
    pub size: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub aspect: Option<f64>,
    pub audio_encode: Option<String>,
    pub audio_bitrate: Option<String>,
    pub audio_channel: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetaSub {
    // For Image, Gifv, Video
    pub width: u32,
    pub height: u32,
    pub size: String,
    pub aspect: f64,

    // For Gifv, Video
    pub frame_rate: Option<String>,

    // For Audio, Gifv, Video
    pub duration: Option<f64>,
    pub bitrate: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Focus {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub enum AttachmentType {
    Image,
    Gifv,
    Video,
    Audio,
    Unknown,
}

impl fmt::Display for AttachmentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttachmentType::Image => write!(f, "image"),
            AttachmentType::Gifv => write!(f, "gifv"),
            AttachmentType::Video => write!(f, "video"),
            AttachmentType::Audio => write!(f, "audio"),
            AttachmentType::Unknown => write!(f, "unknown"),
        }
    }
}

impl FromStr for AttachmentType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "image" => Ok(AttachmentType::Image),
            "gifv" => Ok(AttachmentType::Gifv),
            "video" => Ok(AttachmentType::Video),
            "audio" => Ok(AttachmentType::Audio),
            _ => Ok(AttachmentType::Unknown),
        }
    }
}

impl ser::Serialize for AttachmentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> de::Deserialize<'de> for AttachmentType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match AttachmentType::from_str(s.as_str()) {
            Ok(r) => Ok(r),
            Err(e) => Err(de::Error::custom(e)),
        }
    }
}
