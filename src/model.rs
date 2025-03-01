use std::{fmt::Display, str::FromStr};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub status: Option<StatusType>,
    pub due_date: Option<NaiveDate>,
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Todo {{ id: {}, description: \"{}\", status: {}, due_date: {} }}",
            self.id,
            self.description,
            self.status
                .map_or_else(|| "None".to_string(), |s| s.to_string()),
            self.due_date
                .map_or_else(|| "None".to_string(), |d| d.to_string())
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StatusType {
    Pending = 0,
    InProgress = 1,
    Done = 2,
}

impl Display for StatusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusType::Pending => write!(f, "pending"),
            StatusType::InProgress => write!(f, "in progress"),
            StatusType::Done => write!(f, "done"),
        }
    }
}

impl StatusType {
    pub fn as_db_value(&self) -> i32 {
        match &self {
            StatusType::Pending => 0,
            StatusType::InProgress => 1,
            StatusType::Done => 2,
        }
    }

    pub fn from_db_value(value: i64) -> Option<StatusType> {
        match value {
            0 => Some(StatusType::Pending),
            1 => Some(StatusType::InProgress),
            2 => Some(StatusType::Done),
            _ => None,
        }
    }
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

// Convert from `NaiveDate` to `String` for SQLite
pub fn naive_date_to_db(value: Option<NaiveDate>) -> Option<String> {
    value.map(|d| d.to_string()) // Convert NaiveDate to String in format YYYY-MM-DD
}

// Convert from `String` to `NaiveDate`
pub fn naive_date_from_db(value: Option<String>) -> Option<NaiveDate> {
    value.map(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").unwrap()) // Handle parsing
}
