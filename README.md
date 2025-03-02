# Rustodoo

rutodoo is a simple command-line to-do application that helps you manage your tasks efficiently. It stores tasks in an SQLite database and provides an intuitive command set for task management.

## Installation

Clone the repository and build the application.

## Clone the repository

```bash

git clone https://github.com/LucasMartinsVieira/rustodoo.git
cd rutodoo
cargo build --release
```

## Usage

The application is executed using the rt binary and supports the following commands:

Add a new task

`rt add --description "Buy groceries"`

Adds a new todo item to the list.

List all tasks in a table format

`rt list`

Displays all the todos in the database.

Remove a task

`rt remove <task_id>`

Removes the specified task using its unique ID.

Complete a task

`rt complete <task_id>`

Marks a task as completed.

Reset the todo database

`rt reset`

Clears all tasks from the database.

## Database

The application uses an SQLite database to store tasks. The database file is automatically created in the application's directory.

## License

MIT License

