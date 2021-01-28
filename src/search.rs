use actix_web::web::Path;
use actix_web::HttpResponse;
//use chrono::{DateTime, Utc};
//use uuid::Uuid;

//use crate::response::Response;
use rusted_cypher::cypher::result::Row;
use rusted_cypher::{GraphClient, Statement};

/// search by name /search`
#[get("/search/{name}")]
pub async fn search(_name: Path<(String,)>) -> HttpResponse {
    use crate::constants::{APPLICATION_JSON, NEO4J_DATABASE, NEO4J_ENDPOINT};
    let graph = GraphClient::connect(NEO4J_ENDPOINT, NEO4J_DATABASE).unwrap();

    let statement = Statement::new("match (p:Person) return p.name, p.id;");
    let results = graph.exec(statement).unwrap();

    let mut appender: String = String::from("");
    for row in results.rows() {
        let name: String = row.get("p.name").unwrap();
        //println!("name: {}", name);
        appender = format!("{{'name':'{{{}}}' }}", name.clone());
    }

    HttpResponse::Created()
        .content_type(APPLICATION_JSON)
        .json(appender.to_string())
}
