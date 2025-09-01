-- Add migration script here
CREATE TABLE categories (
  id serial PRIMARY KEY,
  name text not null,
  created_at timestamp default now()
);
