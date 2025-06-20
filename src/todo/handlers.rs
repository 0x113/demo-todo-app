use crate::utils::state::AppState;
use actix_web::{HttpResponse, delete, get, post, web};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct NewTodo {
    title: String,
}

#[post("/api/v1/tasks")]
async fn create(item: web::Json<NewTodo>, data: web::Data<AppState>) -> HttpResponse {
    match data.todo_service.create(&item.title).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[post("/api/v1/tasks/{id}/done")]
async fn mark_as_done(path: web::Path<i32>, data: web::Data<AppState>) -> HttpResponse {
    match data.todo_service.mark_as_done(path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[delete("/api/v1/tasks/{id}")]
async fn delete(path: web::Path<i32>, data: web::Data<AppState>) -> HttpResponse {
    match data.todo_service.delete(path.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[get("/api/v1/tasks")]
async fn list(data: web::Data<AppState>) -> HttpResponse {
    match data.todo_service.list().await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
