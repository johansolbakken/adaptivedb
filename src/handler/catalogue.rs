use http_body_util::{combinators::BoxBody, BodyExt};
use hyper::{body::Bytes, Method, Request, Response, StatusCode};

use crate::{empty, full};

pub async fn catalogue_handler(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/catalogue") => post_catalogue(req).await,
        // Return 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

async fn post_catalogue(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let body = req.into_body().collect().await?.to_bytes();
    let body = String::from_utf8_lossy(&body).to_string();
    tracing::info!("Received body: {}", body);
    Ok(Response::new(full("You've hit the POST /catalogue route!")))
}
