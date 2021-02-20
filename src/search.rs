use actix_web::web::Path;
use actix_web::HttpResponse;
//use chrono::{DateTime, Utc};
//use uuid::Uuid;
use serde::{Deserialize, Serialize};
//use crate::response::Response;
use crate::constants::{APPLICATION_JSON, NEO4J_DATABASE, NEO4J_ENDPOINT};
use rusted_cypher::{GraphClient, Statement};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchRequest {
    pub name: String,
}

const QUERY :&'static str = "MATCH path = (a:Person{fullName:$fullName})-[b]->(c)<-[e]-(f) RETURN nodes(path) AS nodes, relationships(path) AS rels";
/// search by name /search{fullName}`  for now must match the node name
#[get("/search/{name}")]
pub async fn search(query: Path<SearchRequest>) -> HttpResponse {
    let graph =
        GraphClient::connect(NEO4J_ENDPOINT.to_string(), NEO4J_DATABASE.to_string()).unwrap();

    let name = query.name.clone();
    let name = name.as_str();
    let statement = Statement::new(QUERY).with_param("fullName", name).unwrap();

    let results = graph.exec(statement).unwrap();

    HttpResponse::Created()
        .content_type(APPLICATION_JSON.to_string())
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
