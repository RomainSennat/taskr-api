// Import crate and mod
use crate::structs::{AppState, Task};

use actix_web::{guard, web, HttpResponse, Scope};
use bson::oid::ObjectId;
use mongodb::{cursor::Cursor, db::ThreadedDatabase};
use mongodb::coll::options::{FindOneAndUpdateOptions};
use mongodb::coll::options::ReturnDocument::After;
use bson::ordered::OrderedDocument;

// Add task to database
fn add_task(body: web::Json<Task>, state: web::Data<AppState>) -> HttpResponse {
    let document: OrderedDocument = body.to_doc();
    match state.db_client.collection("tasks").insert_one(document.clone(), None) {
        Ok(_val) => HttpResponse::Ok().content_type("application/json").json(document),
        Err(_e) => HttpResponse::InternalServerError().finish()
    }
}

// Fetch all tasks from database
fn fetch_all_tasks(state: web::Data<AppState>) -> HttpResponse {
    let cursor: Cursor = state.db_client.collection("tasks").find(None, None).unwrap();
    let documents: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
    HttpResponse::Ok().content_type("application/json").json(documents)
}

// Fetch one task from database
fn fetch_task(path: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    match ObjectId::with_string(&path) {
        Ok(val) => {
            match state.db_client.collection("tasks").find_one(Some(doc! { "_id" => val }), None) {
                Ok(val) => match val {
                    Some(val) => HttpResponse::Ok().content_type("application/json").json(val),
                    None => HttpResponse::InternalServerError().finish()
                },
                Err(_e) => HttpResponse::InternalServerError().finish()
            }
        }
        Err(_e) => HttpResponse::InternalServerError().finish()
    }
}

// Remove task from database
fn remove_task(path: web::Path<String>, state: web::Data<AppState>) -> HttpResponse {
    match ObjectId::with_string(&path) {
        Ok(val) => {
            match state.db_client.collection("tasks").find_one_and_delete(doc! { "_id" => val }, None) {
                Ok(val) => match val {
                    Some(val) => HttpResponse::Ok().content_type("application/json").json(val),
                    None => HttpResponse::InternalServerError().finish()
                },
                Err(_e) => HttpResponse::InternalServerError().finish()
            }
        }
        Err(_e) => HttpResponse::InternalServerError().finish()
    }
}

// Update task from database
fn update_task(path: web::Path<String>, body: web::Json<Task>, state: web::Data<AppState>) -> HttpResponse {
    match ObjectId::with_string(&path) {
        Ok(val) => {
            let options: FindOneAndUpdateOptions = FindOneAndUpdateOptions { return_document: Option::from(After), max_time_ms: None, projection: None, sort: None, upsert: None, write_concern: None };
            match state.db_client.collection("tasks").find_one_and_replace(doc! { "_id" => val }, body.to_doc(), Some(options)) {
                Ok(val) => match val {
                    Some(val) => HttpResponse::Ok().content_type("application/json").json(val),
                    None => HttpResponse::InternalServerError().finish()
                },
                Err(_e) => HttpResponse::InternalServerError().finish()
            }
        }
        Err(_e) => HttpResponse::InternalServerError().finish()
    }
}

// Task endpoint builder
pub fn build() -> Scope {
    web::scope("/tasks")
        .route("/", web::get().to(fetch_all_tasks))
        .route("/", web::post().guard(guard::Header("Content-Type", "application/json")).to(add_task))
        .route("/{id}", web::delete().to(remove_task))
        .route("/{id}", web::get().to(fetch_task))
        .route("/{id}", web::put().guard(guard::Header("Content-Type", "application/json")).to(update_task))
}
