use crate::{models::TodoModel, AppState};
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(todo_list_handler);

    conf.service(scope);
}

#[get("/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust, SQLX, MySQL, and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","message": MESSAGE}))
}

#[get("/todos")]
async fn todo_list_handler(data: web::Data<AppState>) -> impl Responder {
    let todos = sqlx::query_as::<_, TodoModel>("SELECT * FROM todos ORDER by id")
        .fetch_all(&data.db)
        .await
        .unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "todos": todos
    });

    HttpResponse::Ok().json(json_response)
}
