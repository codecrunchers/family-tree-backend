use actix_web::web::Path;
use actix_web::HttpResponse;
use rusted_cypher::{GraphClient, Statement};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchRequest {
    pub name: String,
}

const QUERY :&'static str = "MATCH path = (:Person)-[b]->(c)<-[e]-(f) RETURN nodes(path) AS nodes, relationships(path) AS rels";
#[get("/family")]
pub async fn family() -> HttpResponse {
    use crate::constants::{APPLICATION_JSON, NEO4J_DATABASE, NEO4J_ENDPOINT};
    let graph = GraphClient::connect(NEO4J_ENDPOINT, NEO4J_DATABASE).unwrap();

    let statement = Statement::new(QUERY);
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
