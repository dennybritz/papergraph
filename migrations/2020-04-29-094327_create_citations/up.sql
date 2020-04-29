CREATE TABLE citations (
  from_paper VARCHAR NOT NULL,
  to_paper VARCHAR NOT NULL,
  PRIMARY KEY(from_paper, to_paper)
);