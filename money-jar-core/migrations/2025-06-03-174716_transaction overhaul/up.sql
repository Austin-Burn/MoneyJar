-- Your SQL goes here
ALTER TABLE Transactions RENAME COLUMN event_id TO item_id;

CREATE TABLE Items (
    id TEXT PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    cost INTEGER NOT NULL,
    payment_progress INTEGER NOT NULL,
    total INTEGER NOT NULL,
    recurring BOOLEAN NOT NULL,
    iteration_count INTEGER NOT NULL,
    event_id VARCHAR(255) NOT NULL,
    FOREIGN KEY (event_id) REFERENCES Events(id) ON DELETE CASCADE
);

ALTER TABLE Users ADD COLUMN balance INTEGER NOT NULL DEFAULT 0;

-- Change Transactions id to TEXT
ALTER TABLE Transactions RENAME TO Transactions_old;
CREATE TABLE Transactions (
    id TEXT PRIMARY KEY NOT NULL,
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

CREATE TABLE PayBatches (
    id VARCHAR(255) NOT NULL,
    transaction_id VARCHAR(255) NOT NULL,
    date VARCHAR(255) NOT NULL,
    PRIMARY KEY (id, transaction_id),
    FOREIGN KEY (transaction_id) REFERENCES Transactions(id)
);



