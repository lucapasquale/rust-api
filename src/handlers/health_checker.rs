use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/healthchecker")]
pub async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, MySQL, and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}
