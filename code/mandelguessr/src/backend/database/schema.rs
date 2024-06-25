// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Int4,
        username -> Varchar,
        score -> Int4,
    }
}

diesel::table! {
    users (username) {
        username -> Varchar,
        password -> Text,
    }
}

diesel::joinable!(games -> users (username));

diesel::allow_tables_to_appear_in_same_query!(
    games,
    users,
);
