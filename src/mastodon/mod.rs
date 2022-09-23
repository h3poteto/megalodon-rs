//! Mastodon related modules

mod api_client;
mod entities;
/// Mastodon API client.
pub mod mastodon;
mod oauth;
mod web_socket;

pub use mastodon::Mastodon;
