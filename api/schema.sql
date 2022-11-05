DROP TABLE IF EXISTS users;

CREATE TABLE users (
  id serial PRIMARY KEY,
  username VARCHAR(100),
  password_hash VARCHAR(100),
  data json
);

CREATE TABLE tasks (
  title VARCHAR(100),
  description VARCHAR(100)
)
