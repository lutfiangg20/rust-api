-- Add migration script here
CREATE TABLE users (
  id serial PRIMARY KEY,
  name text not null,
  email text unique not null,
  password text not null,
  created_at timestamp default now()
);

