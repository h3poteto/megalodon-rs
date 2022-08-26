use super::{Account, Stats, URLs};

pub struct Instance {
    uri: String,
    title: String,
    description: String,
    email: String,
    version: String,
    thumbnail: Option<String>,
    urls: URLs,
    stats: Stats,
    languages: Vec<String>,
    contact_account: Option<Account>,
    max_toot_chars: Option<usize>,
    registrations: Option<bool>,
    configuration: Option<InstanceConfig>,
}

pub struct InstanceConfig {
    statuses: Statuses,
    media_attachments: MediaAttachments,
    polls: Polls,
}

struct Statuses {
    max_characters: u32,
    max_media_attachments: u32,
    characters_reserved_per_url: u32,
}

struct MediaAttachments {
    supported_mime_types: Vec<String>,
    image_size_limit: u32,
    image_matrix_limit: u32,
    video_size_limit: u32,
    video_frame_limit: u32,
    video_matrix_limit: u32,
}

struct Polls {
    max_options: u32,
    max_characters_per_option: u32,
    min_expiration: u32,
    max_expiration: u32,
}
