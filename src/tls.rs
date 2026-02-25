use tokio_tungstenite::Connector;

#[cfg(feature = "rustls-tls")]
pub fn build_connector() -> Option<Connector> {
    use std::sync::Arc;

    let mut root_store = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().certs {
        let _ = root_store.add(cert);
    }
    let config = rustls::ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_safe_default_protocol_versions()
    .expect("Failed to set TLS protocol versions")
    .with_root_certificates(root_store)
    .with_no_client_auth();

    Some(Connector::Rustls(Arc::new(config)))
}

#[cfg(not(feature = "rustls-tls"))]
pub fn build_connector() -> Option<Connector> {
    None
}
