use super::schema::{authors, citations, paper_authors, papers};
use diesel::{Identifiable, Insertable, Queryable};

#[derive(Debug, Identifiable, Insertable, Queryable, AsChangeset)]
#[table_name = "papers"]
pub struct Paper<'a> {
    pub id: &'a str,
    pub title: &'a str,
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
pub struct Author<'a> {
    pub id: &'a str,
    pub name: &'a str,
}

#[derive(Debug, Insertable, Queryable, AsChangeset)]
#[table_name = "paper_authors"]
pub struct PaperAuthor<'a> {
    pub author_id: &'a str,
    pub paper_id: &'a str,
}
