CREATE TABLE invitations (
  id UUID NOT NULL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  department INTEGER,
  is_admin BOOLEAN NOT NULL,
  expires_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL
);
