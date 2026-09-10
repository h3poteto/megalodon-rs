//! Default values

/// A static value for no redirect url.
/// Please use this value when you get an access token.
pub static NO_REDIRECT: &str = "urn:ietf:wg:oauth:2.0:oob";
/// Default scopes value for register app.
pub static DEFAULT_SCOPES: &'static [&str] = &["read", "write", "follow"];
