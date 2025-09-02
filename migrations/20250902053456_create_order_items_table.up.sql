-- Add up migration script here
CREATE TABLE order_items (
  id SERIAL PRIMARY KEY,
  order_id INT REFERENCES orders(id) ON DELETE CASCADE,
  product_id INT REFERENCES products(id),
  quantity INT NOT NULL,
  price INT NOT NULL,
  created_at TIMESTAMP DEFAULT now()
);
