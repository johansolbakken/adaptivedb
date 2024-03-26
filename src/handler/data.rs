use http_body_util::{combinators::BoxBody, BodyExt};
use hyper::{body::Bytes, Method, Request, Response, StatusCode};

use crate::{empty, full, queryprocessing};

pub async fn data_handler(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/data") => post_data(req).await,
        // Return 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::new(empty());
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[derive(Debug, serde::Deserialize)]
struct PostData {
    query: String,
}

async fn post_data(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let body = req.into_body().collect().await?.to_bytes();
    let body_json = serde_json::from_slice::<PostData>(&body);
    if body_json.is_err() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(full(body_json.as_ref().unwrap_err().to_string()))
            .unwrap());
    }

    let body_json = body_json.unwrap();
    let query = body_json.query;
    
    let statement = queryprocessing::dml::parse(&query);
    if statement.is_none() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(full("Invalid query"))
            .unwrap());
    }

    let statement = statement.unwrap();
    let result = queryprocessing::execute(statement);
    let result = serde_json::to_string(&result).unwrap();

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(full(result))
        .unwrap())
}
