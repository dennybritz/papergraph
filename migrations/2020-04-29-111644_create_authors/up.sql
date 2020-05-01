CREATE TABLE authors (
  id VARCHAR PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE paper_authors (
  author_id VARCHAR NOT NULL,
  paper_id VARCHAR NOT NULL,
  PRIMARY KEY(author_id, paper_id)
);

CREATE INDEX paper_authors_author_id_idx ON paper_authors (author_id);
CREATE INDEX paper_authors_paper_id_idx ON paper_authors (paper_id);