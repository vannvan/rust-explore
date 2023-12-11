-- Your SQL goes here
CREATE TABLE tasks (
  id INTEGER NOT NULL PRIMARY KEY,
  task_name TEXT NOT NULL,
  task_start_time TEXT NOT NULL,
  task_end_time TEXT NOT NULL,
  finished BOOLEAN NOT NULL DEFAULT 0
)