//! Gotosocial related modules

mod api_client;
mod entities;
/// Gotosocial API client.
pub mod gotosocial;
mod oauth;
mod web_socket;

pub use gotosocial::Gotosocial;
