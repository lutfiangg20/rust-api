-- Add up migration script here
CREATE TABLE orders (
  id serial PRIMARY KEY,
  user_id int not null,
  status varchar(20) default 'pending', -- pending, paid, shipped, completed, cancelled 
  created_at timestamp default now(),
  constraint fk_user
    FOREIGN KEY(user_id) 
    REFERENCES users(id)
    on delete cascade
);
