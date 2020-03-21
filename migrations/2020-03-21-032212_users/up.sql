CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TYPE cluster_role AS ENUM ('cluster_admin', 'department_admin', 'lessee');

CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  email VARCHAR(100) UNIQUE NOT NULL,
  name VARCHAR(20) NOT NULL,
  password TEXT NOT NULL, -- argon hash
  role cluster_role NOT NULL,
  belong_to INTEGER REFERENCES departments(id),
  created_at TIMESTAMP NOT NULL DEFAULT current_timestamp,
  updated_at TIMESTAMP
);
