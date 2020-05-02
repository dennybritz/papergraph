use super::schema::{authors, paper_authors, papers};
use diesel::{Identifiable, Insertable, Queryable};

#[derive(Debug, Identifiable, Insertable, Queryable, AsChangeset)]
#[table_name = "papers"]
pub struct Paper<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub year: Option<i16>,
    pub paper_abstract: &'a str,
    pub fields_of_study: &'a Vec<String>,
    pub entities: &'a Vec<String>,
    pub pdf_urls: &'a Vec<String>,
    pub s2_url: &'a str,
    pub doi: &'a str,
    pub doi_url: &'a str,    
    pub in_citations: &'a Vec<String>,
    pub out_citations: &'a Vec<String>,
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
