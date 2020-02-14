CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

 CREATE TABLE users (
   id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
   email VARCHAR(100) UNIQUE NOT NULL,
   name VARCHAR(20) NOT NULL,
   password TEXT NOT NULL, -- argon hash
   created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
   updated_at TIMESTAMP
);
