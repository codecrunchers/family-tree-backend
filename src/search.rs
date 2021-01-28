use actix_web::web::Json;
use actix_web::HttpResponse;
//use chrono::{DateTime, Utc};
//use uuid::Uuid;

use crate::constants::APPLICATION_JSON;
//use crate::response::Response;

/// search by name /search`
#[post("/search")]
pub async fn search(tweet_req: Json<String>) -> HttpResponse {
    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json("[]".to_string())
}
