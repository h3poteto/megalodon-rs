use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Token {
    access_token: String,
    token_type: String,
    scope: String,
    created_at: u64,
}

impl Into<MegalodonEntities::Token> for Token {
    fn into(self) -> MegalodonEntities::Token {
        MegalodonEntities::Token {
            access_token: self.access_token,
            token_type: self.token_type,
            scope: self.scope,
            created_at: self.created_at,
        }
    }
}
