CREATE TABLE registries (
  id SERIAL PRIMARY KEY,
  belong_to UUID,
  repo VARCHAR(30) NOT NULL,
  is_public BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE INDEX idx_registry_user_id on registries(belong_to);
