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
    let statement =
        Statement::new("match (p:Person { name: $name }) return p.pid, p.name, p.dob, p.bio;")
            .with_param("name", name)
            .unwrap();

    let results = graph.exec(statement).unwrap();

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        //let stmt = Statement::new("MATCH n RETURN n");
        let WEAK = true;
        assert_eq!(true, WEAK);
    }
}
