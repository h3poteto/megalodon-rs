use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Token {
    access_token: String,
    token_type: String,
    scope: String,
    created_at: u64,
}

impl From<Token> for MegalodonEntities::Token {
    fn from(val: Token) -> Self {
        MegalodonEntities::Token {
            access_token: val.access_token,
            token_type: val.token_type,
            scope: val.scope,
            created_at: val.created_at,
        }
    }
}
