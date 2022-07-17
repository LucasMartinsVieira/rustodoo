use std::fs::OpenOptions;
use std::io::prelude::Read;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use std::{env, process};

pub struct Todo {
    pub todo: Vec<String>,
    pub todo_path: String,
}

impl Todo {
    pub fn new() -> Result<Self, String> {
        let todo_path: String = match env::var("TODO_PATH") {
            Ok(t) => t,
            Err(_) => {
                let home = env::var("HOME").unwrap();

                let legacy_todo = format!("{}/TODO", &home);

                match Path::new(&legacy_todo).exists() {
                    true => legacy_todo,
                    false => format!("{}/.rustodoo", &home),
                }
            }
        };
        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(&todo_path)
            .expect("Couldn't open the todofile");

        let mut buf_reader = BufReader::new(&todofile);

        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents).unwrap();

        let todo = contents.to_string().lines().map(str::to_string).collect();

        Ok(Self { todo, todo_path })
    }

    pub fn list(&self) {
        for (number, task) in self.todo.iter().enumerate() {
            if task.len() > 5 {
                let number = (number + 1).to_string();

                let task = &task[..];

                println!("{} {}", number, task);
            }
        }
    }

    pub fn add(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("rustodoo add takes at least 1 argument");
            process::exit(1);
        }
        let todofile = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(todofile);
        for arg in args {
            if arg.trim().is_empty() {
                continue;
            }

            let line = format!("{}\n", arg);
            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }

    pub fn remove(&self, args: &[String]) {
        if args.is_empty() {
            eprintln!("rustodoo rm takes at least 1 argument");
            process::exit(1);
        }
        let todofile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todo file");

        let mut buffer = BufWriter::new(todofile);

        for (pos, line) in self.todo.iter().enumerate() {
            if args.contains(&(pos + 1).to_string()) {
                continue;
            }

            let line = format!("{}\n", line);

            buffer
                .write_all(line.as_bytes())
                .expect("unable to write data");
        }
    }

    pub fn reset(self) {
        let todofile = OpenOptions::new()
            .write(true)
            .read(true)
            .truncate(true)
            .open(&self.todo_path)
            .expect("Couldn't open the todofile");

        let mut buffer = BufWriter::new(todofile);

        buffer
            .write("".as_bytes())
            .expect("unable to reset the todo list");
    }
    pub fn help(self) {
        println!("rustodoo [options]");
        println!("\tadd   → add a task!.");
        println!("\trm    → remove a task!.");
        println!("\tlist  → list all task!");
        println!("\tdone  → done a task!.");
        println!("\treset → reset all the tasks!.")
    }
}
