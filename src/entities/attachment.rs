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

pub struct Focus {
    pub x: f64,
    pub y: f64,
}

pub enum AttachmentType {
    Image,
    Gifv,
    Video,
    Audio,
    Unknown,
}
