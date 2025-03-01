-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
  id          INTEGER PRIMARY KEY NOT NULL,
  description TEXT    NOT NULL,
  status      TEXT    CHECK (status in ('pending', 'in_progress', 'done')) DEFAULT 'pending',
  due_date    TEXT
);
