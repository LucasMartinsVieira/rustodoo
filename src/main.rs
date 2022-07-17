use rustodoo::Todo;
use std::env;

fn main() {
    let todo = Todo::new().expect("Couldn't create the todo instance");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let command = &args[1];
        match &command[..] {
            "list" => todo.list(),
            "add" => todo.add(&args[2..]),
            "rm" | "remove" => todo.remove(&args[2..]),
            "reset" => todo.reset(),
            "help" | _ => todo.help(),
        }
    } else {
        notice(&args[0]);
    }
}

fn notice(program: &str) {
    println!("Run: '{} help' for more details.", program);
}
