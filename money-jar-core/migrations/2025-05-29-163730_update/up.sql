-- Your SQL goes here
ALTER TABLE Transactions ADD COLUMN payment_method TEXT NOT NULL;
ALTER TABLE Transactions ADD COLUMN comment TEXT;