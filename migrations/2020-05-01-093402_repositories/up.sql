CREATE TABLE repositories (
  id SERIAL PRIMARY KEY,
  belong_to UUID,
  repo_name VARCHAR(30) NOT NULL UNIQUE,
  is_public BOOLEAN NOT NULL DEFAULT 'f',
  is_valid BOOLEAN NOT NULL DeFAULT 't'
);

CREATE INDEX idx_belong_to ON repositories (belong_to);
