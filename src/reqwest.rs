use bytes::Bytes;


#[derive(Debug)]
pub enum SendError {
    Reqwest(reqwest::Error),
    Http(http::Error),
    Builder
}

pub async fn send<I: super::Instance>(
    client: &reqwest::Client,
    input: I
) -> Result<I::Output, SendError> {
    let request = input.into_request().map_err(SendError::Http)?;
    let request = request.try_into().map_err(SendError::Reqwest)?;
    let res = client.execute(request).await.map_err(SendError::Reqwest)?;
    let mut builder = http::response::Builder::new()
        .status(res.status())
        .version(res.version());
    builder.headers_mut().ok_or(SendError::Builder)?.clone_from(res.headers());
    builder.extensions_mut().ok_or(SendError::Builder)?.clone_from(res.extensions());
    let res = builder
        .body(res.bytes().await.map_err(SendError::Reqwest)?)
        .map_err(SendError::Http)?;
    let (head, body) = res.into_parts();
    Ok(I::output(&head, body.as_ref()))
}