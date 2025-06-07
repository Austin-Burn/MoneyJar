-- This file should undo anything in `up.sql`

-- Create a temporary table with the old schema
CREATE TABLE Items_old (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
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

-- Copy data from the new table to the old one
INSERT INTO Items_old (id, name, description, cost, payment_progress, total, recurring, iteration_count, event_id)
SELECT CAST(id AS INTEGER), name, description, cost, payment_progress, total, recurring, iteration_count, event_id
FROM Items;

-- Drop the new table
DROP TABLE Items;

-- Rename the old table to the original name
ALTER TABLE Items_old RENAME TO Items;
