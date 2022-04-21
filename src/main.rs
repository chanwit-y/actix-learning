pub mod db_connection;
pub mod handlers;
pub mod models;
pub mod schema;
pub mod errors;
pub mod utils;

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
extern crate jsonwebtoken as jwt;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate dotenv_codegen;

extern crate actix;
extern crate actix_web;
extern crate futures;

use actix_web::{web, App, HttpServer};
use db_connection::establish_connetion;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .data(establish_connetion())
            .service(
                web::resource("/products")
                    .route(web::get().to(handlers::products::index))
                    .route(web::post().to(handlers::products::create)),
            )
            .service(
                web::resource("/products/{id}")
                    .route(web::get().to(handlers::products::show))
                    .route(web::delete().to(handlers::products::destroy))
                    .route(web::patch().to(handlers::products::update)),
            )
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .run()
    .await
}
