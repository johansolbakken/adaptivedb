use http_body_util::Full;
use http_body_util::{combinators::BoxBody, BodyExt};

use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::StatusCode;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;

use once_cell::sync::Lazy;
use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio::sync::Mutex;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod catalogue;
mod handler;
mod queryprocessing;

async fn handler(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    if req.uri().path().starts_with("/catalogue") {
        return handler::catalogue::catalogue_handler(req).await;
    }
    if req.uri().path().starts_with("/data") {
        return handler::data::data_handler(req).await;
    }

    let mut not_found = Response::new(empty());
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok(not_found)
}

// We create some utility functions to make Empty and Full bodies
// fit our broadened Response body type.
fn empty() -> BoxBody<Bytes, hyper::Error> {
    Full::new(Bytes::new())
        .map_err(|never| match never {})
        .boxed()
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

static CATALOGUE: Lazy<Mutex<catalogue::Catalogue>> = Lazy::new(|| {
    let catalogue =
        catalogue::Catalogue::load().unwrap_or_else(|_| catalogue::Catalogue::new(Vec::new()));
    Mutex::new(catalogue)
});

pub fn get_catalogue() -> &'static Mutex<catalogue::Catalogue> {
    &CATALOGUE
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    info!("AdaptiveDB listening on http://{}", addr);
    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(handler))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
