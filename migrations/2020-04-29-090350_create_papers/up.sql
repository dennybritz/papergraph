CREATE TABLE papers (
  id VARCHAR PRIMARY KEY,
  title TEXT NOT NULL,
  year SMALLINT,
  paper_abstract TEXT,
  entities TEXT[],
  fields_of_study TEXT[],
  pdf_urls TEXT[],
  doi TEXT,
  doi_url TEXT,
  s2_url TEXT,
  in_citations TEXT[],
  out_citations TEXT[]
);

CREATE INDEX title_trgm_idx ON papers USING GIST (title gist_trgm_ops);
CREATE INDEX title_idx ON papers(title);
