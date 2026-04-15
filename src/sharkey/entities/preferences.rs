use super::UserDetail;
use crate::entities as MegalodonEntities;

pub fn convert_preferences(
    user_detail: UserDetail,
    visibility: MegalodonEntities::StatusVisibility,
) -> MegalodonEntities::Preferences {
    let nsfw = if let Some(n) = user_detail.always_mark_nsfw {
        n
    } else {
        false
    };

    MegalodonEntities::Preferences {
        posting_default_visibility: visibility,
        posting_default_sensitive: nsfw,
        posting_default_language: user_detail.lang,
        reading_expand_media: MegalodonEntities::preferences::ExpandMedia::Default,
        reading_expand_spoilers: false,
    }
}
