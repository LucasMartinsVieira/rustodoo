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
            "done" => todo.done(&args[2..]),
            "reset" => todo.reset(),
            "help" | _ => help(),
        }
    } else {
        notice(&args[0]);
    }
}

fn notice(program: &str) {
    println!("Run: '{} help' for more details.", program);
}

fn help() {
    println!("rustodoo [options]");
    println!("\tadd   → add a task!.");
    println!("\trm    → remove a task!.");
    println!("\tlist  → list all task!");
    println!("\tdone  → done a task!.");
    println!("\treset → reset all the tasks!.")
}
