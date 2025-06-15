use crate::todo::{model::Todo, repository};

#[derive(Clone)]
pub struct TodoService {
    db_pool: sqlx::PgPool,
}

impl TodoService {
    pub async fn new(db_pool: sqlx::PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, title: &str) -> Result<Todo, sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;
        let todo = repository::create(&mut tx, title).await?;
        tx.commit().await?;
        Ok(todo)
    }

    pub async fn mark_as_done(&self, id: i32) -> Result<(), sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;
        repository::mark_as_done(&mut tx, id).await?;
        tx.commit().await?;
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<Todo>, sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;
        let todos = repository::list(&mut tx).await?;
        tx.commit().await?;
        Ok(todos)
    }

    pub async fn delete(&self, id: i32) -> Result<(), sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;
        repository::delete(&mut tx, id).await?;
        tx.commit().await?;
        Ok(())
    }
}
