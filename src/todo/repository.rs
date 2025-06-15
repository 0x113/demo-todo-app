use crate::todo::model::Todo;
use sqlx::PgConnection;

pub async fn create(conn: &mut PgConnection, title: &str) -> Result<Todo, sqlx::Error> {
    let todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title) VALUES ($1) RETURNING id, title, completed",
    )
    .bind(title)
    .fetch_one(conn)
    .await?;
    Ok(todo)
}

pub async fn mark_as_done(conn: &mut PgConnection, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE todos SET completed = TRUE WHERE id = $1")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn list(conn: &mut PgConnection) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos")
        .fetch_all(conn)
        .await?;
    Ok(todos)
}

pub async fn delete(conn: &mut PgConnection, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
}
