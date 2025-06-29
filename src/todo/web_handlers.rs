use crate::{
    todo,
    utils::{env, state::AppState},
};
use actix_web::{HttpResponse, Result, get, post, web};
use serde::Deserialize;
use tera::{Context, Tera};

#[derive(Deserialize)]
pub struct CreateTodoForm {
    title: String,
}

async fn render_todos(
    todo_service: &todo::service::TodoService,
    tmpl: &Tera,
    context: &mut Context,
) -> Result<HttpResponse> {
    match todo_service.list().await {
        Ok(todos) => {
            context.insert("todos", &todos);
        }
        Err(e) => {
            context.insert("error", &format!("Failed to load todos: {}", e));
        }
    }

    context.insert("pod_name", &env::read_with_default("POD_NAME", "Unknown"));
    context.insert("namespace", &env::read_with_default("NAMESPACE", "Unknown"));
    context.insert("node_name", &env::read_with_default("NODE_NAME", "Unknown"));

    let rendered = tmpl.render("index.html", &context).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[get("/")]
pub async fn index(data: web::Data<AppState>, tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = Context::new();
    render_todos(&data.todo_service, &tmpl, &mut context).await
}

#[post("/")]
pub async fn create_todo(
    form: web::Form<CreateTodoForm>,
    data: web::Data<AppState>,
    tmpl: web::Data<Tera>,
) -> Result<HttpResponse> {
    let mut context = Context::new();

    // Create the new todo
    match data.todo_service.create(&form.title).await {
        Ok(_) => {
            context.insert("success", "Task added successfully!");
        }
        Err(e) => {
            context.insert("error", &format!("Failed to add task: {}", e));
        }
    }
    render_todos(&data.todo_service, &tmpl, &mut context).await
}

#[post("/complete/{id}")]
pub async fn complete_todo(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> Result<HttpResponse> {
    let todo_id = path.into_inner();

    match data.todo_service.mark_as_done(todo_id).await {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish()),
        Err(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/?error=Failed to complete task"))
            .finish()),
    }
}

#[post("/delete/{id}")]
pub async fn delete_todo(path: web::Path<i32>, data: web::Data<AppState>) -> Result<HttpResponse> {
    let todo_id = path.into_inner();

    match data.todo_service.delete(todo_id).await {
        Ok(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/"))
            .finish()),
        Err(_) => Ok(HttpResponse::Found()
            .append_header(("Location", "/?error=Failed to delete task"))
            .finish()),
    }
}
