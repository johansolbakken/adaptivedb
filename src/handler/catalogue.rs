use http_body_util::{combinators::BoxBody, BodyExt};
use hyper::{body::Bytes, Method, Request, Response, StatusCode};

use crate::{catalogue, empty, full, queryprocessing};

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

#[derive(Debug, serde::Deserialize)]
struct PostTable {
    schema: String,
}

#[derive(Debug, serde::Serialize)]
struct PostTableErrorResponse {
    errors: Vec<String>,
}

async fn post_catalogue(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let body = req.into_body().collect().await?.to_bytes();
    let body_json = serde_json::from_slice::<PostTable>(&body);
    if body_json.is_err() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(full(body_json.as_ref().unwrap_err().to_string()))
            .unwrap());
    }

    let body_json = body_json.unwrap();
    let schema = body_json.schema;

    let models = queryprocessing::ddl::parse(schema);
    let errors = queryprocessing::ddl::analyze(&models);
    if !errors.is_empty() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(full(
                serde_json::to_string(&PostTableErrorResponse { errors }).unwrap(),
            ))
            .unwrap());
    }

    let mut catalogue = crate::get_catalogue().lock().await;
    for model in models.iter() {
        if catalogue.table_exists(&model.name) {
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(full(format!(
                    "Aborted. Table {} already exists",
                    model.name
                )))
                .unwrap());
        }
    }

    for model in models.iter() {
        let columns = model
            .fields
            .iter()
            .map(|field| {
                catalogue::Column::new(
                    field.name.clone(),
                    field.field_type.clone(),
                    field.is_nullable,
                )
            })
            .collect();
        let primary_key_index = model
            .fields
            .iter()
            .position(|field| field.is_primary_key)
            .expect("Analyzing should have caught this error");
        let table = catalogue::Table::new(model.name.clone(), columns, primary_key_index as u32);
        catalogue.add_table(table);
    }

    catalogue.save().unwrap();

    let model_names: Vec<String> = models.iter().map(|model| model.name.clone()).collect();
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(full(format!("Created following tables: {:?}", model_names)))
        .unwrap())
}
