use actix_web::web::Path;
use actix_web::HttpResponse;
//use chrono::{DateTime, Utc};
//use uuid::Uuid;
use serde::{Deserialize, Serialize};
//use crate::response::Response;
use rusted_cypher::{GraphClient, Statement};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchRequest {
    pub name: String,
}

/// search by name /search{name}`  for now must match the node name
#[get("/search/{name}")]
pub async fn search(query: Path<SearchRequest>) -> HttpResponse {
    use crate::constants::{APPLICATION_JSON, NEO4J_DATABASE, NEO4J_ENDPOINT};
    let graph = GraphClient::connect(NEO4J_ENDPOINT, NEO4J_DATABASE).unwrap();

    let name = query.name.clone();
    let name = name.as_str();
    let statement = Statement::new("MATCH (p:Person { name: $name }) return p.name")
        .with_param("name", name)
        .unwrap();

    let results = graph.exec(statement).unwrap();

    let serialized = serde_json::to_string(&results).unwrap();

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(serialized)
}
