-- Your SQL goes here
CREATE TABLE users (
       email VARCHAR(100) NOT NULL PRIMARY KEY,
       name VARCHAR(20) NOT NULL,
       hash VARCHAR(122) NOT NULL, -- argon hash
       created_at TIMESTAMP NOT NULL
);
