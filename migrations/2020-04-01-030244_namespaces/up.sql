CREATE TABLE namespaces (
  id SERIAL PRIMARY KEY,
  uid UUID NOT NULL,
  namespace VARCHAR(30) NOT NULL,
  valid BOOLEAN NOT NULL DEFAULT 't'
);

CREATE INDEX idx_user_uuid ON namespaces (uid);
