use std::{collections::HashSet, fmt, marker};

use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{
    market::{Market, MarketType, Outcome},
    odds::Odds,
};

#[derive(Deserialize, Debug, Clone)]
pub struct Bookmaker {
    pub key: String,
    pub title: String,
    last_update: DateTime<Utc>,
    pub markets: Vec<Market>,
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
impl Bookmaker {
    pub fn get_enabled_markets(&self) -> Vec<Market> {
        let to_exclude = [MarketType::OutrightsLay, MarketType::H2hLay];
        return self
            .markets
            .clone()
            .into_iter()
            .filter(|x| !to_exclude.contains(&x.key))
            .collect();
    }

    pub fn get_odds(&self, market_key: &MarketType, outcome_key: &str) -> Option<Odds> {
        for market in &self.markets {
            if market.key == *market_key {
                return market.odds_for_outcome(outcome_key);
            }
        }
        return None;
    }

    pub fn get_over_under_line(&self) -> Option<f64> {
        for market in &self.markets {
            if market.key == MarketType::Totals {
                let first_outcome_option = market.outcomes.first();
                let first_outcome = match first_outcome_option {
                    None => return None,
                    Some(x) => x,
                };
                return first_outcome.point;
            }
        }
        return None;
    }

    pub fn get_offered_outcomes(&self, market: &MarketType) -> Vec<Outcome> {
        let specified_market = self.markets.iter().find(|x| x.key == *market);

        match specified_market {
            Some(x) => return x.outcomes.clone(),
            None => return Vec::new(),
        };
    }
}
