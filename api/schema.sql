DROP TABLE IF EXISTS users;

CREATE TABLE users (
  id serial PRIMARY KEY,
  username VARCHAR(100),
  password_hash VARCHAR(100)
);