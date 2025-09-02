-- Add migration script here
CREATE TABLE products (
  id serial PRIMARY KEY,
  name text not null,
  price int not null,
  category_id int not null,
  created_at timestamp default now(),
  constraint fk_category
    FOREIGN KEY(category_id) 
    REFERENCES categories(id)
    on delete cascade 
);

CREATE INDEX idx_products_category_id on products(category_id);
