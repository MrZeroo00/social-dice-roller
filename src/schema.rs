table! {
    players (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    rooms (id) {
        id -> Text,
        created_at -> Timestamp,
    }
}
