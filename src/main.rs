use actix_web::{App, HttpServer, web};
use tera::Tera;
use todo_app_rs::{todo, utils};
use tracing::info;

async fn init_tables(db_pool: &sqlx::PgPool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id SERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE
        )",
    )
    .execute(db_pool)
    .await
    .unwrap();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    // Read config from env
    let host = std::env::var("HOST").expect("HOST is not set");
    let port = std::env::var("PORT")
        .expect("PORT is not set")
        .parse::<u16>()
        .unwrap();
    let db_host = std::env::var("DB_HOST").expect("DB_HOST is not set");
    let db_port = std::env::var("DB_PORT").expect("DB_PORT is not set");
    let db_user = std::env::var("DB_USER").expect("DB_USER is not set");
    let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD is not set");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME is not set");

    info!("Starting server at http://{host}:{port}");

    info!("Connecting to database at {db_host}:{db_port}/{db_name}");
    let db_pool = sqlx::PgPool::connect(&format!(
        "postgres://{db_user}:{db_password}@{db_host}:{db_port}/{db_name}"
    ))
    .await
    .unwrap();
    info!("Connected to database");

    info!("Initializing tables...");
    init_tables(&db_pool).await;
    info!("Tables initialized");

    let todo_service = todo::service::TodoService::new(db_pool).await;

    // Initialize Tera
    let tera = match Tera::new("src/templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(utils::state::AppState {
                todo_service: todo_service.clone(),
            }))
            .app_data(web::Data::new(tera.clone()))
            // Web UI routes
            .service(todo::web_handlers::index)
            .service(todo::web_handlers::create_todo)
            .service(todo::web_handlers::complete_todo)
            .service(todo::web_handlers::delete_todo)
            // API routes
            .service(todo::handlers::create)
            .service(todo::handlers::list)
            .service(todo::handlers::mark_as_done)
            .service(todo::handlers::delete)
    })
    .bind((host, port))?
    .run()
    .await
}
