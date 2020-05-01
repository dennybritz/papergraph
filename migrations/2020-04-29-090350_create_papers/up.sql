CREATE TABLE papers (
  id VARCHAR PRIMARY KEY,
  title TEXT NOT NULL,
  year SMALLINT,
  in_citations TEXT[],
  out_citations TEXT[]
)