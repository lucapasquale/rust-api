use crate::{
    models::TodoModel,
    schema::{CreateTodoSchema, FilterOptions},
    AppState,
};
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/todos")]
pub async fn todo_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos = sqlx::query_as!(
        TodoModel,
        "SELECT * FROM todos ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    HttpResponse::Ok().json(json!({ "todos": todos }))
}

#[post("/todos")]
pub async fn create_todo_handler(
    body: web::Json<CreateTodoSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = sqlx::query_as!(
        TodoModel,
        "INSERT INTO todos (title, content, done) VALUES ($1, $2, FALSE) RETURNING *",
        body.title.to_string(),
        body.content.to_owned().unwrap_or_default(),
    )
    .fetch_one(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    match query_result {
        Ok(todo) => return HttpResponse::Ok().json(json!({ "todo": todo })),

        Err(err) => {
            return HttpResponse::InternalServerError()
                .json(json!({ "message": format!("{:?}", err)
                }));
        }
    }
}
