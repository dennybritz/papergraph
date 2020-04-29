table! {
    authors (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

table! {
    citations (from_paper, to_paper) {
        from_paper -> Varchar,
        to_paper -> Varchar,
    }
}

table! {
    paper_authors (author_id, paper_id) {
        author_id -> Varchar,
        paper_id -> Varchar,
    }
}

table! {
    papers (id) {
        id -> Varchar,
        title -> Text,
        year -> Nullable<Int2>,
    }
}

allow_tables_to_appear_in_same_query!(authors, citations, paper_authors, papers,);
