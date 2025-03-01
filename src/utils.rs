use prettytable::{format::consts, row, Cell, Row, Table};

use crate::model::{StatusType, Todo};

pub fn display_table(todos: &[Todo]) {
    let mut table = Table::new();

    table.set_format(*consts::FORMAT_BOX_CHARS);

    table.add_row(row!["ID", "Description", "Status", "Due Date"]);

    for todo in todos {
        table.add_row(Row::new(vec![
            Cell::new(&todo.id.to_string()),
            Cell::new(&todo.description),
            Cell::new(format_status(&todo.status)),
            Cell::new(&todo.due_date.map_or("None".to_string(), |d| d.to_string())),
        ]));
    }

    table.printstd();
}

fn format_status(status: &Option<StatusType>) -> &str {
    if let Some(s) = status {
        match s {
            StatusType::Pending => "❌",
            StatusType::InProgress => "⏳",
            StatusType::Done => "✅",
        }
    } else {
        "None"
    }
}
