use std::process;

use clap::Parser;
use cli::{Cli, SubCommand};
use db::Database;
use repository::SqliteTodoRepository;
use service::TodoService;
use utils::display_table;

mod cli;
mod db;
mod model;
mod repository;
mod service;
mod utils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let db = Database::new().await?;
    let pool = db.pool().clone();

    let todo_repository = SqliteTodoRepository::new(pool);
    let todo_service = TodoService::new(&todo_repository);

    match &cli.subcommand {
        SubCommand::Add(args) => {
            match todo_service
                .create_todo(args.description.as_str(), args.status, args.due_date)
                .await
            {
                Ok(_) => {
                    println!("Successfully added todo!");
                }
                Err(e) => {
                    eprintln!("Error adding todo: {}", e);
                    process::exit(1);
                }
            }
        }
        SubCommand::List => match todo_service.list_todos().await {
            Ok(todos) => {
                if todos.is_empty() {
                    println!("No todos found.");
                    process::exit(1);
                }

                display_table(&todos);
            }
            Err(e) => {
                eprintln!("Error retrieving todos: {}", e);
                process::exit(1);
            }
        },
        SubCommand::Remove(args) => match todo_service.remove_todo(args.id).await {
            Ok(id) => {
                println!("Successfully removing todo with id: {}", id);
            }

            Err(e) => {
                eprintln!("Error removing todo: {}", e);
                process::exit(1);
            }
        },
        SubCommand::Reset => match todo_service.reset_todos().await {
            Ok(todos_deleted) => {
                if todos_deleted == 0 {
                    println!("No todos were found!");
                    process::exit(0);
                }

                println!("Successfully reset todos! {} todos deleted.", todos_deleted);
            }

            Err(e) => {
                eprintln!("Error reseting todos: {}", e);
                process::exit(1);
            }
        },
        SubCommand::Complete(args) => match todo_service.complete_todo(args.id).await {
            Ok(id) => {
                println!("Successfully completed todo with ID: {}", id);
            }

            Err(e) => {
                eprintln!("Error completing todo: {}", e);
                process::exit(1);
            }
        },
    };

    Ok(())
}
