//! Friendica related modules

mod api_client;
mod entities;
pub mod friendica;
mod oauth;
#[cfg(feature = "streaming")]
mod web_socket;

pub use friendica::Friendica;
