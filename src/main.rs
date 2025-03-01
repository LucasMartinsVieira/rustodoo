use clap::Parser;
use cli::{Cli, SubCommand};
use rustodoo::Todo;
use std::env;

mod cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let todo = Todo::new().expect("Couldn't create the todo instance");

    let cli = Cli::parse();

    let args: Vec<String> = env::args().collect();

    match &cli.subcommand {
        SubCommand::Add(args) => {
            println!("{}", args.description)
        }
        SubCommand::List => todo!(),
        SubCommand::Remove(args) => todo!(),
    }

    // if args.len() > 1 {
    //     let command = &args[1];
    //     match &command[..] {
    //         "list" => todo.list(),
    //         "add" => todo.add(&args[2..]),
    //         "rm" | "remove" => todo.remove(&args[2..]),
    //         "reset" => todo.reset(),
    //         "help" | _ => todo.help(),
    //     }
    // } else {
    //     notice(&args[0]);
    // }
    Ok(())
}
