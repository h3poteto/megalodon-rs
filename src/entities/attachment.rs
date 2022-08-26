pub struct Attachment {
    id: String,
    r#type: AttachmentType,
    url: String,
    remote_url: Option<String>,
    preview_url: String,
    text_url: Option<String>,
    meta: Option<AttachmentMeta>,
    description: Option<String>,
    blurhash: Option<String>,
}

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

pub struct MetaSub {
    // For Image, Gifv, Video
    width: u32,
    height: u32,
    size: String,
    aspect: f64,

    // For Gifv, Video
    frame_rate: Option<String>,

    // For Audio, Gifv, Video
    duration: Option<f64>,
    bitrate: Option<u32>,
}

pub struct Focus {
    x: f64,
    y: f64,
}

pub enum AttachmentType {
    Image,
    Gifv,
    Video,
    Audio,
    Unknown,
}
