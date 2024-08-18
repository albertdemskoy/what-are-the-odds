use std::fmt;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use strum_macros::EnumIter;

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    id: String,
    sport_key: String,
    sport_title: String,
    commence_time: DateTime<Utc>,
    home_team: String,
    away_team: String,
    bookmakers: Vec<Bookmaker>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Bookmaker {
    pub key: String,
    pub title: String,
    last_update: DateTime<Utc>,
    pub markets: Vec<Market>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Market {
    pub key: MarketType,
    pub outcomes: Vec<Outcome>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum MarketType {
    H2h,
    H2hLay,
    Spreads,
    Totals,
    Outrights,
    OutrightsLay,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Sport {
    pub key: String,
    pub group: String,
    pub title: String,
    description: String,
    active: bool,
    has_outrights: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Outcome {
    pub name: String,
    pub price: f64,
    pub point: Option<f64>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Region {
    Us,
    Us2,
    Uk,
    Au,
    Eu,
}

impl fmt::Display for MarketType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MarketType::H2h => write!(f, "h2h"),
            MarketType::H2hLay => write!(f, "h2h_lay"),
            MarketType::Spreads => write!(f, "spreads"),
            MarketType::Totals => write!(f, "totals"),
            MarketType::Outrights => write!(f, "outrights"),
            MarketType::OutrightsLay => write!(f, "outrights_lay"),
        }
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Region::Us => write!(f, "us"),
            Region::Us2 => write!(f, "us2"),
            Region::Uk => write!(f, "uk"),
            Region::Au => write!(f, "au"),
            Region::Eu => write!(f, "eu"),
        }
    }
}
