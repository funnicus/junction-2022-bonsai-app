DROP TABLE IF EXISTS users, tasks;

CREATE TABLE users (
  id serial PRIMARY KEY,
  username VARCHAR(100),
  password_hash VARCHAR(100),
  data json,
  completed_tasks integer[],
  quiz_results json
);

CREATE TABLE tasks (
  id serial PRIMARY KEY,
  title VARCHAR(100),
  category VARCHAR(100),
  description VARCHAR(100)
)
