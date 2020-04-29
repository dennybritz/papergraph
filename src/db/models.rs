use super::schema::papers;
use super::schema::citations;
use diesel::{Queryable, Insertable, Identifiable};

#[derive(Debug, Identifiable, Insertable, Queryable, AsChangeset)]
#[table_name="papers"]
pub struct Paper {
    pub id: String,
    pub title: String,
    pub year: Option<i16>,
}

#[derive(Debug, Insertable, Queryable, AsChangeset)]
#[table_name="citations"]
pub struct Citation<'a> {
    pub from_paper: &'a str,
    pub to_paper: &'a str,
}
