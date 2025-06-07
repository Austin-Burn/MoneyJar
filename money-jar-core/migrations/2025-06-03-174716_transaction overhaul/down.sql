-- This file should undo anything in `up.sql`
DROP TABLE PayBatches;

-- Revert Transactions id to INTEGER
ALTER TABLE Transactions RENAME TO Transactions_old;
CREATE TABLE Transactions (
    id INTEGER PRIMARY KEY NOT NULL,
    from_user_id VARCHAR(255) NOT NULL,
    to_user_id VARCHAR(255) NOT NULL,
    item_id VARCHAR(255) NOT NULL,
    amount INTEGER NOT NULL,
    date VARCHAR(255) NOT NULL,
    payment_method VARCHAR(255) NOT NULL,
    comment VARCHAR(255),
    FOREIGN KEY (from_user_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (to_user_id) REFERENCES Users(id) ON DELETE CASCADE,
    FOREIGN KEY (item_id) REFERENCES Items(id) ON DELETE CASCADE
);
DROP TABLE Transactions_old;

ALTER TABLE Users DROP COLUMN balance;

DROP TABLE Items;

ALTER TABLE Transactions RENAME COLUMN item_id TO event_id;

