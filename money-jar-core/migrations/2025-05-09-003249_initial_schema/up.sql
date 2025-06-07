-- Your SQL goes here

CREATE TABLE Users (
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    phone VARCHAR(255)
);

CREATE TABLE UserPaymentMethods (
    user_id VARCHAR(255) NOT NULL REFERENCES Users(id) ON DELETE CASCADE,
    payment_method VARCHAR(255) NOT NULL,
    PRIMARY KEY (user_id, payment_method)
);

CREATE TABLE Events (
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    owner_id VARCHAR(255) NOT NULL REFERENCES Users(id),
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255),
    event_date VARCHAR(255),
    reoccuring BOOLEAN NOT NULL,
    reoccuring_interval VARCHAR(255),
    final_date VARCHAR(255)
);

--keeps track of who is in what event User and Event purposes work directly inversely
CREATE TABLE WhoInWhat (
    user_id VARCHAR(255) NOT NULL REFERENCES Users(id) ON DELETE CASCADE,
    event_id VARCHAR(255) NOT NULL REFERENCES Events(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, event_id)
);


CREATE TABLE Transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    from_user_id VARCHAR(255) NOT NULL REFERENCES Users(id),
    to_user_id VARCHAR(255) NOT NULL REFERENCES Users(id),
    event_id VARCHAR(255) NOT NULL REFERENCES Events(id),
    amount INTEGER NOT NULL,
    date VARCHAR(255) NOT NULL
);

