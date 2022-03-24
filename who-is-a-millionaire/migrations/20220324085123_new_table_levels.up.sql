-- Add up migration script here
CREATE TABLE levels (
  id uuid DEFAULT uuid_generate_v4 (),
  level INT NOT NULL,
  label VARCHAR(50) NOT NULL,
  PRIMARY KEY (id)
)