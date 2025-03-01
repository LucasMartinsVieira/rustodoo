use chrono::NaiveDate;
use clap::{Parser, Subcommand};

use crate::model::StatusType;

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
    /// Reset database of todos
    #[command()]
    Reset,
    /// Completes a todo
    #[command(alias = "c")]
    Complete(CompleteArgs),
}

#[derive(Parser, Debug)]
pub struct AddArgs {
    #[arg(short, long)]
    /// Description of the todo
    pub description: String,
    #[arg(short, long)]
    /// Status of the todo
    pub status: Option<StatusType>,
    #[arg(short('D'), long, value_parser = parse_due_date)]
    /// Due date of the todo
    pub due_date: Option<NaiveDate>,
}

#[derive(Parser)]
pub struct RemoveArgs {
    pub id: i64,
}

#[derive(Parser)]
pub struct CompleteArgs {
    pub id: i64,
}

fn parse_due_date(s: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
}
