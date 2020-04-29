use diesel::pg::PgConnection;
use diesel::prelude::*;

pub use super::{models, schema};

pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn insert_from_s2_record(
    conn: &PgConnection,
    record: &crate::io::Paper,
) -> Result<(), diesel::result::Error> {
    let paper = models::Paper {
        id: &record.id,
        title: &record.title,
        year: record.year.map(|y| y as i16),
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

    let out_citations = record
        .out_citations
        .iter()
        .map(|to_paper| models::Citation {
            from_paper: &record.id,
            to_paper: to_paper,
        });
    let in_citations = record
        .in_citations
        .iter()
        .map(|from_paper| models::Citation {
            from_paper: from_paper,
            to_paper: &record.id,
        });
    let citations: Vec<models::Citation> = out_citations.chain(in_citations).collect();

    // TODO: Should we upsert here?
    diesel::insert_into(schema::papers::table)
        .values(&paper)
        .on_conflict(schema::papers::id)
        .do_nothing()
        .execute(conn)?;

    diesel::insert_into(schema::authors::table)
        .values(&authors)
        .on_conflict(schema::authors::id)
        .do_nothing()
        .execute(conn)?;

    diesel::insert_into(schema::paper_authors::table)
        .values(&paper_authors)
        .on_conflict((
            schema::paper_authors::paper_id,
            schema::paper_authors::author_id,
        ))
        .do_nothing()
        .execute(conn)?;

    diesel::insert_into(schema::citations::table)
        .values(&citations)
        .on_conflict((schema::citations::from_paper, schema::citations::to_paper))
        .do_nothing()
        .execute(conn)?;

    log::debug!(
        "inserted paper {} [{} citations, {} authors]",
        &record.id,
        citations.len(),
        authors.len()
    );

    Ok(())
}
