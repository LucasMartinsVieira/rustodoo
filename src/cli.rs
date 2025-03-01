use std::str::FromStr;

use chrono::NaiveDate;
use clap::{Parser, Subcommand};

/// A simple todo rust application
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: SubCommand,
}

#[derive(Subcommand)]
pub enum SubCommand {
    /// Add a new Todo
    #[command(alias = "a")]
    Add(AddArgs),
    /// List all todos
    #[command(alias = "l")]
    List,
    /// Remove a todo
    #[command(alias = "r")]
    Remove(RemoveArgs),
}

#[derive(Parser, Debug)]
pub struct AddArgs {
    #[arg(short, long)]
    /// Description of the todo
    pub description: String,
    #[arg(short, long)]
    /// Status of the todo
    pub status: Option<StatusType>,
    #[arg(short, long, value_parser = parse_due_date)]
    /// Due date of the todo
    pub due_date: Option<NaiveDate>,
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

#[derive(Parser)]
pub struct RemoveArgs {
    pub id: i64,
}

#[derive(Debug, Clone)]
pub enum StatusType {
    Pending,
    InProgress,
    Done,
}

fn parse_due_date(s: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}
