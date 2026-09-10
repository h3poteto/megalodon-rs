//! Pixelfed related modules

mod api_client;
mod entities;
mod oauth;
/// Pixelfed API client.
pub mod pixelfed;
#[cfg(feature = "streaming")]
mod web_socket;

pub use pixelfed::Pixelfed;
