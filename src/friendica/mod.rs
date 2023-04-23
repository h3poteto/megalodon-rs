//! Friendica related modules

mod api_client;
mod entities;
pub mod friendica;
mod oauth;
mod web_socket;

pub use friendica::Friendica;
