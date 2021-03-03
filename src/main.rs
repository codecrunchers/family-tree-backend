#[macro_use]
extern crate actix_web;
use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpServer};
use std::{env, io};

mod constants;
mod family;
mod response;
mod search;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    // add specific origin to allowed origin list
                    .allowed_origin("http://family.snarfel.com:3000")
                    // set allowed methods list
                    .allowed_methods(vec!["GET", "POST"])
                    // set allowed request header list
                    .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
                    // add header to allowed list
                    .allowed_header(header::CONTENT_TYPE)
                    // set list of headers that are safe to expose
                    .expose_headers(&[header::CONTENT_DISPOSITION])
                    // set CORS rules ttl
                    .max_age(3600),
            )
            // register HTTP requests handlers
            .service(search::search)
            .service(family::family)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}
