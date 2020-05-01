CREATE FUNCTION cites(paper_row papers, limit_ integer)
RETURNS SETOF papers AS $$
  SELECT p2.* FROM papers p1
  JOIN papers p2 ON p2.id=ANY(p1.out_citations)
  WHERE p1.id = paper_row.id AND p2.id != paper_row.id
  LIMIT limit_
$$ LANGUAGE SQL STABLE;


CREATE FUNCTION num_citations(paper_row papers)
RETURNS integer AS $$
  SELECT array_length(in_citations, 1) FROM papers 
  WHERE id = paper_row.id
$$ LANGUAGE SQL STABLE;