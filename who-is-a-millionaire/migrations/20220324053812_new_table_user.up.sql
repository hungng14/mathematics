CREATE TABLE users (
  id uuid DEFAULT uuid_generate_v4 (),
  fullname VARCHAR (50) NOT NULL,
  age INT,
  PRIMARY KEY (id)
)

-- CREATE TABLE contacts (
--     contact_id uuid DEFAULT uuid_generate_v4 (),
--     first_name VARCHAR NOT NULL,
--     last_name VARCHAR NOT NULL,
--     email VARCHAR NOT NULL,
--     phone VARCHAR,
--     PRIMARY KEY (contact_id)
-- );