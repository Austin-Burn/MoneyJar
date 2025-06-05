-- This file should undo anything in `up.sql`
ALTER TABLE Users DROP COLUMN balance;

ALTER TABLE Transactions RENAME COLUMN item_id TO event_id;

DROP TABLE Items;

