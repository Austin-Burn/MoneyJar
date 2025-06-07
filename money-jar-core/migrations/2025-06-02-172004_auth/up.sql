-- Your SQL goes here
CREATE TABLE Auth
(
    user_id VARCHAR(255) NOT NULL REFERENCES Users (id) ON DELETE CASCADE,
    token   VARCHAR(255) NOT NULL,
    expiry  TEXT         NOT NULL,
    PRIMARY KEY (user_id)
);