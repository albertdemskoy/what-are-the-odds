// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        book_key -> Text,
        book_title -> Text,
        region -> Text,
        is_exchange -> Bool,
    }
}

diesel::table! {
    events (id) {
        id -> Int4,
        sport_id -> Int4,
        home_team -> Text,
        away_team -> Text,
        commence_time -> Timestamptz,
    }
}

diesel::table! {
    odds_offering (id) {
        id -> Int4,
        event_id -> Int4,
        book_id -> Int4,
        offered_at -> Timestamptz,
        outcome -> Text,
        market_type -> Text,
        offered_line -> Nullable<Numeric>,
        offered_odds -> Numeric,
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
diesel::joinable!(odds_offering -> books (book_id));
diesel::joinable!(odds_offering -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    books,
    events,
    odds_offering,
    sports,
);
