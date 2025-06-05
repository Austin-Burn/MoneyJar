-- Your SQL goes here
ALTER TABLE Transactions RENAME COLUMN event_id TO item_id NOT NULL REFERENCES Items(id);

CREATE TABLE Items (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ON DELETE CASCADE event_id REFERENCES Events(id),
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    price INTEGER NOT NULL,
    cost INTEGER NOT NULL,
    payment_progress INTEGER NOT NULL,
    event_id INTEGER NOT NULL REFERENCES Events(id)
);

ALTER TABLE Users ADD COLUMN balance INTEGER NOT NULL;


