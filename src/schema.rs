table! {
    post (id) {
        id -> Int4,
        title -> Varchar,
        publish_date -> Nullable<Date>,
        content -> Nullable<Text>,
    }
}
