CREATE TABLE departments(
  id SERIAL PRIMARY KEY,
  name VARCHAR(100) UNIQUE NOT NULL,
  admin UUID
);

INSERT INTO departments(id, name) values(1, 'default');
INSERT INTO departments(id, name) values(2, 'test');
