use async_trait::async_trait;
use chrono::NaiveDate;
use sqlx::{query, SqlitePool};

use crate::model::{naive_date_from_db, naive_date_to_db, StatusType, Todo};

#[async_trait]
pub trait TodoRepository {
    async fn create_todo(
        &self,
        description: &str,
        status: Option<StatusType>,
        due_date: Option<NaiveDate>,
    ) -> sqlx::Result<Todo>;
}

pub struct SqliteTodoRepository {
    pool: SqlitePool,
}

impl SqliteTodoRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for SqliteTodoRepository {
    async fn create_todo(
        &self,
        description: &str,
        status: Option<StatusType>,
        due_date: Option<NaiveDate>,
    ) -> sqlx::Result<Todo> {
        let status_value = status.map(|s| s.to_db_value());
        let due_date_str = naive_date_to_db(due_date);

        let db_result = query!(
            "INSERT INTO todos (description, status, due_date) VALUES (?, ?, ?) RETURNING id, description, status, due_date",
            description, status_value, due_date_str)
        .fetch_one(&self.pool)
        .await?;

        let todo = Todo {
            id: db_result.id,
            description: db_result.description,
            status: db_result
                .status
                .map(StatusType::from_db_value)
                .unwrap_or_else(|| Some(StatusType::Pending)),
            due_date: naive_date_from_db(db_result.due_date),
        };

        Ok(todo)
    }
}
