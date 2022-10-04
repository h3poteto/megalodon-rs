//! Pleroma related modules

mod api_client;
mod entities;
mod oauth;
pub mod pleroma;
mod web_socket;

pub use pleroma::Pleroma;
