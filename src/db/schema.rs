table! {
    authors (id) {
        id -> Varchar,
        name -> Varchar,
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
        in_citations -> Nullable<Array<Text>>,
        out_citations -> Nullable<Array<Text>>,
    }
}

allow_tables_to_appear_in_same_query!(
    authors,
    paper_authors,
    papers,
);
