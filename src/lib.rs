pub use bytes::Bytes;
use http::{Request, Response, response};

// pub mod hyper;
pub mod reqwest;

pub trait Instance {
    type Output;
    fn into_request(self) -> http::Result<Request<Bytes>>;
    fn output(head: &response::Parts, body: &[u8]) -> Self::Output;
}
