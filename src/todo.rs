use std::str::FromStr;

use chrono::NaiveDate;

#[derive(Debug)]
pub struct Todo {
    pub description: String,
    pub status: Option<StatusType>,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub enum StatusType {
    Pending,
    InProgress,
    Done,
}

impl FromStr for StatusType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(StatusType::Pending),
            "inprogress" | "in_progress" => Ok(StatusType::InProgress),
            "done" => Ok(StatusType::Done),
            _ => Err(format!(
                "Invalid status: {}. Use 'pending', 'inprogress', or 'done'.",
                s
            )),
        }
    }
}

impl Todo {
    // pub fn new() -> Result<Self, String> {}

    // pub fn list(&self) {}

    pub fn add(todo: Todo) {
        println!("{:?}", todo.description);
        println!("{:?}", todo.status.unwrap_or(StatusType::Pending));
        println!(
            "{:?}",
            todo.due_date.unwrap_or(NaiveDate::from_ymd(2022, 01, 10))
        );
    }

    // pub fn remove(&self, args: &[String]) {}

    // pub fn reset(self) {}
}
