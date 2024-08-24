use std::fmt;

pub mod db;
pub mod requests;
pub mod util;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use strum_macros::EnumIter;

use crate::common::MarketType;

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    id: String,
    pub sport_key: String,
    sport_title: String,
    pub commence_time: DateTime<Utc>,
    pub home_team: String,
    pub away_team: String,
    bookmakers: Option<Vec<Bookmaker>>,
}

#[derive(Deserialize, Debug, Clone)]
struct Bookmaker {
    pub key: String,
    pub title: String,
    last_update: DateTime<Utc>,
    pub markets: Vec<Market>,
}

impl Bookmaker {
    pub fn is_exchange(&self) -> bool {
        let lower_title = self.title.to_lowercase();
        return lower_title.contains("betfair") || lower_title.contains("matchbook");
    }
}

#[derive(Deserialize, Debug, Clone)]
struct Market {
    pub key: MarketType,
    pub outcomes: Vec<Outcome>,
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
    pub price: BigDecimal,
    pub point: Option<BigDecimal>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum Region {
    Us,
    Us2,
    Uk,
    Au,
    Eu,
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

impl Region {
    fn to_common_region(&self) -> crate::common::Region {
        use crate::common::Region as CommonRegion;

        match *self {
            Region::Us => CommonRegion::Us,
            Region::Us2 => CommonRegion::Us,
            Region::Uk => CommonRegion::Uk,
            Region::Au => CommonRegion::Au,
            Region::Eu => CommonRegion::Eu,
        }
    }
}
