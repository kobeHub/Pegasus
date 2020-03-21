CREATE TABLE departments(
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) NOT NULL,
  admin UUID
);
