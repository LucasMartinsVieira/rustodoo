use clap::Parser;
use cli::{Cli, SubCommand};
use todo::Todo;

mod cli;
mod todo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.subcommand {
        SubCommand::Add(args) => {
            let todo = Todo {
                description: args.description.clone(),
                status: args.status.clone(),
                due_date: args.due_date,
            };

            Todo::add(todo);
        }
        SubCommand::List => todo!(),
        SubCommand::Remove(args) => todo!(),
        SubCommand::Reset => todo!(),
    }
    Ok(())
}
