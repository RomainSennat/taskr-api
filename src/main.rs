// Import crate and mod
extern crate actix_cors;
extern crate actix_web;
extern crate confy;
#[macro_use(bson, doc)]
extern crate mongodb;
#[macro_use]
extern crate serde_derive;

mod handlers;
mod structs;

use actix_cors::Cors;
use actix_web::{http::{header, Method}, App, HttpServer};
use mongodb::{Client, ThreadedClient};
use mongodb::db::{Database, ThreadedDatabase};
use structs::{AppState, Configuration};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Read API configuration file
    let cfg: Configuration = confy::load("taskr-api").unwrap();
    
    // Connection to database
    let db_client: Database = match Client::connect(cfg.dbpath.as_str(), cfg.dbport) {
        Ok(val) => val.db(cfg.dbname.as_str()),
        Err(_e) => panic!("Failed to initialize mongo connection.")
    };
    
    // Authenticate to database
    if cfg.dbauthenticate {
        match db_client.auth(cfg.dbuser.as_str(), cfg.dbpassword.as_str()) {
            Ok(_val) => (),
            Err(_e) => panic!("Failed on mongo authentication.")
        }
    }
    
    println!("API running on http://localhost:5000");
    
    // Initialize API
    HttpServer::new(move || {
        App::new()
            .data(AppState { db_client: db_client.clone() }) // API state
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:5000")
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec![Method::DELETE, Method::GET, Method::POST, Method::PUT])
                    .allowed_headers(vec![header::ACCEPT, header::AUTHORIZATION, header::CONTENT_TYPE])
                    .max_age(3600)
                    .finish()
            ) // CORS configuration
            .service(handlers::task::build()) // Task endpoint
    })
    .bind("127.0.0.1:5000")
    .unwrap()
    .run()
    .await
}
