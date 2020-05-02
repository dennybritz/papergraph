use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::pg::upsert::excluded;
use itertools::Itertools;

pub use super::{models, schema};

pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url))
}

/// Each json record from Semantic Scholar maps to multiple database rows
/// which are accumulated in a `RecordBatch`
pub struct RecordBatch<'a> {
    pub papers: Vec<models::Paper<'a>>,
    pub authors: Vec<models::Author<'a>>,
    pub paper_authors: Vec<models::PaperAuthor<'a>>,
}

impl<'a> RecordBatch<'a> {
    /// Creates a new empty RecordBatch
    pub fn new() -> Self {
        RecordBatch {
            papers: vec![],
            authors: vec![],
            paper_authors: vec![],
        }
    }

    /// Inserts this RecordBatch into the database
    pub fn insert(&self, conn: &PgConnection) -> Result<(), diesel::result::Error> {
        let chunk_size = 4096;

        log::info!("inserting {} papers", &self.papers.len());
        for papers in &self.papers.iter().chunks(chunk_size) {
            let papers: Vec<&models::Paper<'a>> = papers.collect();
            diesel::insert_into(schema::papers::table)
                .values(papers)
                .on_conflict(schema::papers::id)
                .do_update()
                .set(schema::papers::id.eq(excluded(schema::papers::id)))
                .execute(conn)?;
        }

        log::info!("inserting {} authors", &self.authors.len());
        for chunk in &self.authors.iter().chunks(chunk_size) {
            let chunk: Vec<&models::Author<'a>> = chunk.collect();
            diesel::insert_into(schema::authors::table)
                .values(chunk)
                .on_conflict_do_nothing()
                .execute(conn)?;
        }

        for chunk in &self.paper_authors.iter().chunks(chunk_size) {
            let chunk: Vec<&models::PaperAuthor<'a>> = chunk.collect();
            diesel::insert_into(schema::paper_authors::table)
                .values(chunk)
                .on_conflict_do_nothing()
                .execute(conn)?;
        }

        return Ok(());
    }

    /// Moves all elements of other into this RecordBatch
    pub fn append(&mut self, other: &mut RecordBatch<'a>) {
        self.papers.append(&mut other.papers);
        self.authors.append(&mut other.authors);
        self.paper_authors.append(&mut other.paper_authors);
    }
}

pub fn s2_record_to_batch<'a>(record: &'a crate::io::Paper) -> RecordBatch<'a> {
    let paper = models::Paper {
        id: &record.id,
        title: &record.title,
        year: record.year.map(|y| y as i16),
        paper_abstract: &record.paper_abstract,
        fields_of_study: &record.fields_of_study,
        entities: &record.entities,
        pdf_urls: &record.pdf_urls,
        s2_url: &record.s2_url,
        doi: &record.doi,
        doi_url: &record.doi_url,
        in_citations: &record.in_citations,
        out_citations: &record.out_citations,
    };

    // TODO: Is it correct to filter out authors without ID!?
    let authors: Vec<models::Author> = record
        .authors
        .iter()
        .filter(|a| a.ids.len() > 0)
        .map(|a| models::Author {
            id: a.ids.get(0).unwrap(),
            name: &a.name,
        })
        .collect();

    let paper_authors: Vec<models::PaperAuthor> = authors
        .iter()
        .map(|a| models::PaperAuthor {
            paper_id: &record.id,
            author_id: &a.id,
        })
        .collect();

    RecordBatch {
        papers: vec![paper],
        authors,
        paper_authors,
    }
}
