use serde::{Deserialize, Serialize};

use super::{
    attachment::{AttachmentMeta, AttachmentType},
    Attachment,
};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct AsyncAttachment {
    pub id: String,
    pub r#type: AttachmentType,
    pub url: Option<String>,
    pub remote_url: Option<String>,
    pub preview_url: Option<String>,
    pub text_url: Option<String>,
    pub meta: Option<AttachmentMeta>,
    pub description: Option<String>,
    pub blurhash: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum UploadMedia {
    Attachment(Attachment),
    AsyncAttachment(AsyncAttachment),
}
