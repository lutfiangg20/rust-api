-- Add up migration script here
CREATE TABLE payments (
    id SERIAL PRIMARY KEY,
    order_id INT REFERENCES orders(id),
    amount INT NOT NULL,
    method VARCHAR(50), -- e.g. bank_transfer, credit_card, e-wallet
    status VARCHAR(20) DEFAULT 'pending',
    paid_at TIMESTAMP DEFAULT now()
);
