use actix_web::web::Path;
use actix_web::HttpResponse;
//use chrono::{DateTime, Utc};
//use uuid::Uuid;

use crate::constants::APPLICATION_JSON;
//use crate::response::Response;

/// search by name /search`
#[get("/search/{name}")]
pub async fn search(_name: Path<(String,)>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json("[]".to_string())
}
