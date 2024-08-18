// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Int4,
        sport_id -> Int4,
        home_team -> Text,
        away_team -> Text,
        commence_time -> Timestamp,
    }
}

diesel::table! {
    sports (id) {
        id -> Int4,
        category -> Varchar,
        sport_key -> Varchar,
        title -> Varchar,
    }
}

diesel::joinable!(events -> sports (sport_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    sports,
);
