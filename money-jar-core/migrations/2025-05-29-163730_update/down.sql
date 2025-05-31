-- This file should undo anything in `up.sql`
ALTER TABLE Transactions DROP COLUMN payment_method;
ALTER TABLE Transactions DROP COLUMN comment;
