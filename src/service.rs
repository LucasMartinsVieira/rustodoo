use chrono::NaiveDate;

use crate::{
    model::{StatusType, Todo},
    repository::TodoRepository,
};

pub struct TodoService<'a> {
    repository: &'a dyn TodoRepository,
}

impl<'a> TodoService<'a> {
    pub fn new(repository: &'a dyn TodoRepository) -> Self {
        TodoService { repository }
    }

    pub async fn create_todo(
        &self,
        description: &str,
        status: Option<StatusType>,
        due_date: Option<NaiveDate>,
    ) -> Result<Todo, String> {
        self.repository
            .create_todo(description, status, due_date)
            .await
            .map_err(|e| format!("Failed to create todo: {}", e))
    }

    pub async fn list_todos(&self) -> Result<Vec<Todo>, String> {
        self.repository
            .list_todos()
            .await
            .map_err(|e| format!("Failed to fetch todos: {}", e))
    }

    pub async fn reset_todos(&self) -> Result<u64, String> {
        self.repository
            .reset_todos()
            .await
            .map_err(|e| format!("Failed to reset todos: {}", e))
    }

    pub async fn remove_todo(&self, id: i64) -> Result<i64, String> {
        self.repository
            .remove_todo(id)
            .await
            .map_err(|e| format!("Failed to remove todo: {}", e))
    }

    pub async fn complete_todo(&self, id: i64) -> Result<i64, String> {
        self.repository
            .complete_todo(id)
            .await
            .map_err(|e| format!("Failed to complete todo: {}", e))
    }
}
