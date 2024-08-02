use bytes::Bytes;
use http_body_util::BodyExt;
use hyper::body::Body;
use hyper_tls::HttpsConnector;
use hyper_util::client::{self, legacy::{connect::HttpConnector, Client}};
use tokio::io::{AsyncWriteExt as _, self};

pub enum SendError {
    Hyper(hyper::Error),
    Http(http::Error),
}

pub async fn send<I: super::Instance>(
    sender: &mut hyper::client::conn::http2::SendRequest<Bytes>,
    input: I,
) -> Result<I::Output, SendError> {
    let res = sender
        .send_request(input.into_request().map_err(SendError::Http)?)
        .await
        .map_err(SendError::Hyper)?;
    let (head, body) = res.into_parts();
    let body = body.collect().await.map_err(SendError::Hyper)?.to_bytes();
    Ok(I::output(&head, body))
}

// pub enum TlsSendError {
//     Client(client::legacy::Error),
//     Hyper(hyper::Error),
// }

// pub async fn tls_send<I: super::Instance>(
//     client: &mut Client<HttpsConnector<HttpConnector>, I::Body>,
//     input: I,
// ) -> Result<I::Output, TlsSendError>
// where I::Body: Body + Send + 'static + Unpin,
//     <I::Body as Body>::Data: Send,
//     <I::Body as Body>::Error: Into<Box<dyn std::error::Error + Send + Sync>>
//  {
//     let res = client.request(input.into_request().map_err(TlsSendError::Hyper)?)
//         .await.map_err(TlsSendError::Hyper)?;

//     let (head, body) = res.into_parts();
//     let body = body.collect().await.map_err(|x| ).to_bytes();
//     I::output(&head, body)
// }