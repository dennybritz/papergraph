table! {
    citations (from_paper, to_paper) {
        from_paper -> Varchar,
        to_paper -> Varchar,
    }
}

table! {
    papers (id) {
        id -> Varchar,
        title -> Text,
        year -> Nullable<Int2>,
    }
}

allow_tables_to_appear_in_same_query!(
    citations,
    papers,
);
