-- Your SQL goes here

-- Create a temporary table with the new schema
CREATE TABLE Items_new (
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

-- Copy data from the old table to the new one
INSERT INTO Items_new (id, name, description, cost, payment_progress, total, recurring, iteration_count, event_id)
SELECT CAST(id AS TEXT), name, description, cost, payment_progress, total, recurring, iteration_count, event_id
FROM Items;

-- Drop the old table
DROP TABLE Items;

-- Rename the new table to the original name
ALTER TABLE Items_new RENAME TO Items;
