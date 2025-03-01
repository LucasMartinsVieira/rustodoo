-- Add migration script here
CREATE TABLE IF NOT EXISTS todos (
  id          INTEGER PRIMARY KEY NOT NULL,
  description TEXT    NOT NULL,
  status      INTEGER DEFAULT 0,    
  due_date    TEXT
);
