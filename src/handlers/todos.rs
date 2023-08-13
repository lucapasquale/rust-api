use crate::{
    models::TodoModel,
    schema::{CreateTodoSchema, FilterOptions, UpdateTodoSchema},
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/todos")]
pub async fn todo_list_handler(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = opts.offset.unwrap_or(0);

    let todos = sqlx::query_as!(
        TodoModel,
        "SELECT * FROM todos ORDER by id DESC LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    HttpResponse::Ok().json(json!({ "status": "success", "todos": todos }))
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
    .await;

    match query_result {
        Ok(todo) => {
            return HttpResponse::Ok().json(json!({
                "status": "success", "todo": todo
            }))
        }

        Err(err) => {
            return HttpResponse::InternalServerError().json(json!({
                "status": "error", "message": format!("{:?}", err.to_string())
            }));
        }
    }
}

#[patch("/todos/{id}")]
pub async fn update_todo_handler(
    path: web::Path<i32>,
    body: web::Json<UpdateTodoSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todo_id = path.into_inner();
    let query_result = sqlx::query_as!(TodoModel, "SELECT * FROM todos WHERE id = $1", todo_id)
        .fetch_one(&data.db)
        .await;

    let todo = match query_result {
        Ok(todo) => todo,
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().json(
                serde_json::json!({ "status": "error", "message": format!("Todo with ID: {} not found", todo_id) }),
            );
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    };

    let query_result = sqlx::query_as!(
        TodoModel,
        "UPDATE todos SET title = $2, content = $3, done = $4 WHERE id = $1 RETURNING *",
        todo_id,
        body.title.to_owned().unwrap_or(todo.title.clone()),
        body.content
            .to_owned()
            .unwrap_or(todo.content.clone().unwrap_or_default()),
        body.done.to_owned().unwrap_or(todo.done.clone())
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(updated_todo) => {
            return HttpResponse::Ok().json(json!({
                "status": "success", "todo": updated_todo
            }))
        }

        Err(err) => {
            return HttpResponse::InternalServerError().json(
                json!({ "status": "error", "message": format!("{:?}", err.to_string())
                }),
            );
        }
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo_handler(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todo_id = path.into_inner();

    let _query_result = sqlx::query_as!(
        TodoModel,
        "DELETE FROM todos WHERE id = $1 RETURNING *",
        todo_id
    )
    .fetch_one(&data.db)
    .await;

    HttpResponse::Ok().json(json!({ "status": "success" }))
}
