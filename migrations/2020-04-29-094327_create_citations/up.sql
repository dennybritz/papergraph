CREATE TABLE citations (
  from_paper VARCHAR NOT NULL,
  to_paper VARCHAR NOT NULL,
  PRIMARY KEY(from_paper, to_paper)
);

CREATE INDEX citations_from_paper_idx ON citations (from_paper);
CREATE INDEX citations_to_paper_idx ON citations (to_paper);