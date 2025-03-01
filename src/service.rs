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
}
