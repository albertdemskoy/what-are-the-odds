// @generated automatically by Diesel CLI.

diesel::table! {
    sports (id) {
        id -> Int4,
        category -> Varchar,
        sport_key -> Varchar,
        title -> Varchar,
    }
}
