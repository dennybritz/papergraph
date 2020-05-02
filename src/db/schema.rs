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
        paper_abstract -> Nullable<Text>,
        entities -> Nullable<Array<Text>>,
        fields_of_study -> Nullable<Array<Text>>,
        pdf_urls -> Nullable<Array<Text>>,
        doi -> Nullable<Text>,
        doi_url -> Nullable<Text>,
        s2_url -> Nullable<Text>,
        in_citations -> Nullable<Array<Text>>,
        out_citations -> Nullable<Array<Text>>,
    }
}

allow_tables_to_appear_in_same_query!(
    authors,
    paper_authors,
    papers,
);
