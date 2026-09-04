use crate::error::Error as MegalodonError;
use futures_util::future::BoxFuture;
use reqwest::{multipart::Form, Request};
use tower_service::Service;

/// Generic HTTP client, implemented for [`reqwest::Client`] or anything implementing
/// `for<'a> tower_service::Service<Request, Response>`.
pub trait HttpClient: std::fmt::Debug + Send + Sync + 'static {
    /// Performs an HTTP request.
    fn request<'a>(
        &'a self,
        req: reqwest::Request,
    ) -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>;
    /// Clones the client into a box
    fn box_clone(&self) -> Box<dyn HttpClient>;
}
impl<T> HttpClient for T
where
    for<'a> &'a T: Service<
        reqwest::Request,
        Response = reqwest::Response,
        Error = reqwest::Error,
        Future: Send,
    >,
    T: std::fmt::Debug + Clone + Send + Sync + 'static,
{
    fn request(
        &self,
        req: reqwest::Request,
    ) -> BoxFuture<'_, Result<reqwest::Response, reqwest::Error>> {
        let mut this = self;
        Box::pin(this.call(req))
    }
    fn box_clone(&self) -> Box<dyn HttpClient> {
        Box::new(self.clone())
    }
}
impl Clone for Box<dyn HttpClient> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

/// Sets the body of the request to the JSON serialization of `v`, and adds a `Content-Type` header if missing.
pub(crate) fn set_json_body<T: serde::Serialize>(
    mut req: Request,
    v: &T,
) -> Result<Request, serde_json::Error> {
    let body = serde_json::to_vec(v)?;
    req.headers_mut()
        .entry(reqwest::header::CONTENT_TYPE)
        .or_insert_with(|| reqwest::header::HeaderValue::from_static("application/json"));
    *req.body_mut() = Some(reqwest::Body::from(body));
    Ok(req)
}

/// Sets the body of the request to the multipart form, and sets a `Content-Type` header.
pub(crate) fn set_multipart_body(mut req: Request, form: Form) -> Result<Request, MegalodonError> {
    req.headers_mut().insert(
        reqwest::header::CONTENT_TYPE,
        format!("multipart/form-data; boundary={}", form.boundary()).try_into()?,
    );
    // Content-Length would be good to add, but `form.compute_length` is not public...
    *req.body_mut() = Some(reqwest::Body::wrap_stream(form.into_stream()));
    Ok(req)
}

/// Begins a websocket stream
#[cfg(feature = "streaming")]
pub(crate) async fn begin_websocket(
    client: &dyn HttpClient,
    mut url: reqwest::Url,
    headers: reqwest::header::HeaderMap<reqwest::header::HeaderValue>,
) -> Result<tokio_tungstenite::WebSocketStream<reqwest::Upgraded>, MegalodonError> {
    use tokio_tungstenite::tungstenite::client::IntoClientRequest as _;
    use crate::error::Kind;

    if url.scheme() == "ws" {
        url.set_scheme("http").unwrap();
    } else if url.scheme() == "wss" {
        url.set_scheme("https").unwrap();
    }

    let mut req = (&url)
        .into_client_request()?
        .map(|()| reqwest::Body::default());

    req.headers_mut().extend(headers);
    let accept_key = tokio_tungstenite::tungstenite::handshake::derive_accept_key(
        req.headers()
            .get(http::header::SEC_WEBSOCKET_KEY)
            .unwrap()
            .as_bytes(),
    );
    let res = client.request(req.try_into()?).await?;
    if res.status() != reqwest::StatusCode::SWITCHING_PROTOCOLS {
        return Err(MegalodonError::new_own(
            "Websocket error".to_string(),
            Kind::HTTPStatusError,
            Some(url.to_string()),
            Some(res.status().as_u16()),
            Some(res.headers().clone()),
        ));
    }
    if res
        .headers()
        .get(reqwest::header::CONNECTION)
        .and_then(|v| std::str::from_utf8(v.as_bytes()).ok())
        .map(|v| !v.trim_ascii().eq_ignore_ascii_case("upgrade"))
        .unwrap_or(true)
    {
        return Err(MegalodonError::new_own(
            "Websocket not upgraded".to_string(),
            Kind::UnknownSNSError,
            Some(url.to_string()),
            Some(res.status().as_u16()),
            Some(res.headers().clone()),
        ));
    }
    if res
        .headers()
        .get(reqwest::header::UPGRADE)
        .and_then(|v| std::str::from_utf8(v.as_bytes()).ok())
        .map(|v| !v.trim_ascii().eq_ignore_ascii_case("streaming"))
        .unwrap_or(true)
    {
        return Err(MegalodonError::new_own(
            "Websocket upgraded to wrong protocol".to_string(),
            Kind::UnknownSNSError,
            Some(url.to_string()),
            Some(res.status().as_u16()),
            Some(res.headers().clone()),
        ));
    }
    if res
        .headers()
        .get(reqwest::header::SEC_WEBSOCKET_ACCEPT)
        .map(|v| v.as_bytes())
        != Some(accept_key.as_bytes())
    {
        return Err(MegalodonError::new_own(
            "Incorrect websocket accept".to_string(),
            Kind::UnknownSNSError,
            Some(url.to_string()),
            Some(res.status().as_u16()),
            Some(res.headers().clone()),
        ));
    }
    let stream = res.upgrade().await?;
    Ok(tokio_tungstenite::WebSocketStream::from_raw_socket(
        stream,
        tokio_tungstenite::tungstenite::protocol::Role::Client,
        None,
    )
    .await)
}
