use crate::structs::{AppState, Task};

use actix_web::{guard, web, HttpResponse, Scope};
use bson::oid::ObjectId;
use mongodb::{cursor::Cursor, db::ThreadedDatabase};

fn add_task(body: web::Json<Task>, state: web::Data<AppState>) -> HttpResponse {
    match state.db_client.collection("tasks").insert_one(body.to_doc(), None) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_e) => HttpResponse::InternalServerError().finish()
    }
}

fn fetch_all_tasks(state: web::Data<AppState>) -> HttpResponse {
    let cursor: Cursor = state.db_client.collection("tasks").find(None, None).unwrap();
    let documents: Vec<_> = cursor.map(|doc| doc.unwrap()).collect();
    HttpResponse::Ok().content_type("application/json").json(documents)
}

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

fn update_task(path: web::Path<String>, body: web::Json<Task>, state: web::Data<AppState>,) -> HttpResponse {
    match ObjectId::with_string(&path) {
        Ok(val) => {
            match state.db_client.collection("tasks").find_one_and_replace(doc! { "_id" => val }, body.to_doc(), None) {
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

pub fn build() -> Scope {
    web::scope("/tasks")
        .route("/", web::get().to(fetch_all_tasks))
        .route("/", web::post().guard(guard::Header("Content-Type", "application/json")).to(add_task))
        .route("/{id}", web::delete().to(remove_task))
        .route("/{id}", web::get().to(fetch_task))
        .route("/{id}", web::put().guard(guard::Header("Content-Type", "application/json")).to(update_task))
}
