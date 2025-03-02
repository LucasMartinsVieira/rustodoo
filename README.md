# Rustodoo

Rustodoo is a simple command-line to-do application that helps you manage your tasks efficiently. It stores tasks in an SQLite database and provides an intuitive command set for task management.

## Installation

Clone the repository and build the application.

## Clone the repository

```bash

git clone https://github.com/LucasMartinsVieira/rustodoo.git
cd rutodoo
cargo build --release
```

## Usage

The application is executed using the `rt` binary and supports the following commands:

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

## Architecture

The project is structured as follows:

```
├── Cargo.lock
├── Cargo.toml
├── flake.lock
├── flake.nix
├── LICENSE
├── migrations
│   └── 20250301141157_create_todo_table.sql
├── README.md
└── src
    ├── cli.rs          # Handles command-line argument parsing
    ├── db.rs           # Manages database connection and queries
    ├── main.rs         # Entry point of the application
    ├── model.rs        # Defines data structures and DTOs
    ├── repository.rs   # Encapsulates database operations
    ├── service.rs      # Implements business logic
    └── utils.rs        # Contains helper functions
```

- **CLI Layer**: (cli.rs): Parses and processes user commands.
- **Database Layer**: (db.rs): Handles SQLite interactions.
- **Model Layer**: (model.rs): Defines data structures for tasks.
- **Repository Layer**: (repository.rs): Encapsulates data access logic.
- **Service Layer**: (service.rs): Implements core business logic.
- **Utilities**: (utils.rs): Contains helper functions.
- **Migrations**: Stores SQL schema changes.

## Database

The application uses an SQLite database to store tasks. The database file is automatically created in the application's directory.

## Next steps

- [ ] Add task editing functionality.
- [ ] Implement DTOs for data transfer.
- [ ] Validate CLI arguments.
- [ ] Display task creation times (e.g., "10 hours ago").
- [ ] Allow users to view task details by ID.
- [ ] Add tests 

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

