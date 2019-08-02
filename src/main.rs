extern crate actix_cors;
extern crate actix_web;
#[macro_use(bson, doc)]
extern crate mongodb;
#[macro_use]
extern crate serde_derive;

use actix_cors::Cors;
use actix_web::{http, App, HttpServer};
use mongodb::{Client, ThreadedClient};

mod handlers;
mod structs;

fn main() {
    println!("API running on http://localhost:5000");

    HttpServer::new(|| {
        App::new()
            .data(match Client::connect("localhost", 27017) {
                Ok(val) => structs::AppState { db_client: val.db("taskr") },
                Err(_e) => panic!("Failed to initialize client.")
            })
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:5000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
                    .max_age(3600)
            )
            .service(handlers::task::build())
    })
    .bind("127.0.0.1:5000")
    .unwrap()
    .run()
    .unwrap();
}
