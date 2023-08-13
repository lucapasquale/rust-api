use actix_web::web;

mod health_checker;
mod todos;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker::health_checker_handler)
        .service(todos::todo_list_handler)
        .service(todos::create_todo_handler);

    conf.service(scope);
}
