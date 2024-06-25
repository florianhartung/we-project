CREATE TABLE games (
    id SERIAL PRIMARY KEY,
    username VARCHAR REFERENCES users(username) NOT NULL,
    score INTEGER NOT NULL
);