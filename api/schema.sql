DROP TABLE IF EXISTS users;

CREATE TABLE users (
  id serial PRIMARY KEY,
  username VARCHAR(100),
  password_hash VARCHAR(100),
  data json,
  completedTasks integer[],
  quiz_result json
);

CREATE TABLE tasks (
  id serial PRIMARY KEY,
  title VARCHAR(100),
  category VARCHAR(100)
  description VARCHAR(100)
)
