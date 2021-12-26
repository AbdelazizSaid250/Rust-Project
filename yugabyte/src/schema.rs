table! {
    job (id) {
        id -> Uuid,
        name -> Varchar,
        total_size -> Int4,
        downloaded_size -> Int4,
        percent_downloaded -> Int4,
        status -> Varchar,
        is_active -> Bool,
        creation_date -> Timestamp,
        expiration_date -> Nullable<Timestamp>,
    }
}
