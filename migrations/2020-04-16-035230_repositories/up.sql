CREATE TABLE repositories (
  id SERIAL PRIMARY KEY,
  belong_to UUID,
  repo_name VARCHAR(30) NOT NULL,
  is_public BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE INDEX idx_belong_to ON repositories (belong_to);
