
CREATE TABLE question(
  id uuid DEFAULT uuid_generate_v4 (),
  level INT NOT NULL,
  name VARCHAR(50) NOT NULL,
  PRIMARY KEY(id)
);

CREATE TABLE question_detail(
  id uuid DEFAULT uuid_generate_v4 (),
  question_id uuid NOT NULL,
  option TEXT NOT NULL,
  PRIMARY KEY(id),
  CONSTRAINT fk_question FOREIGN KEY(question_id) REFERENCES question(id)
);
