CREATE TABLE tags (
  id SERIAL PRIMARY KEY,
  repo_name VARCHAR(30) NOT NULL,
  tag_name VARCHAR(30) NOT NULL,
  is_valid BOOLEAN NOT NULL DEFAULT 't'
);

CREATE INDEX idx_image_tag_repo ON tags (repo_name);
