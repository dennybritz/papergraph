use super::schema::{authors, citations, paper_authors, papers};
use diesel::{Identifiable, Insertable, Queryable};

#[derive(Debug, Identifiable, Insertable, Queryable, AsChangeset)]
#[table_name = "papers"]
pub struct Paper {
    pub id: String,
    pub title: String,
    pub year: Option<i16>,
}

#[derive(Debug, Insertable, Queryable, AsChangeset)]
#[table_name = "citations"]
pub struct Citation<'a> {
    pub from_paper: &'a str,
    pub to_paper: &'a str,
}

#[derive(Debug, Identifiable, Insertable, Queryable, AsChangeset)]
#[table_name = "authors"]
pub struct Author {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Insertable, Queryable, AsChangeset)]
#[table_name = "paper_authors"]
pub struct PaperAuthor<'a> {
    pub author_id: &'a str,
    pub paper_id: &'a str,
}
