use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Debug)]
pub struct Sport {
    key: String,
    group: String,
    title: String,
    description: String,
    active: bool,
    has_outrights: bool
}

#[derive(Deserialize, Debug)]
pub struct Event {
    id: String,
    sport_key: String,
    commence_time: DateTime<Utc>,
    home_team: String, 
    away_team: String,
    bookmakers: Vec<Bookmaker>
}

#[derive(Deserialize, Debug)]
pub struct Bookmaker {
    key: String,
    title: String,
    last_update: DateTime<Utc>,
    markets: Vec<Market>
}

#[derive(Deserialize, Debug)]
pub struct Market {
    key: String,
    outcomes: Vec<Outcome>
}

#[derive(Deserialize, Debug)]
pub struct Outcome {
    name: String,
    price: i32,
    point: Option<f32>
}



