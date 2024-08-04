use std::{collections::HashSet, fmt, marker};

use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{
    market::{Market, MarketType},
    odds::Odds,
};

#[derive(Deserialize, Debug, Clone)]
pub struct Bookmaker {
    pub key: String,
    title: String,
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

    pub fn get_offered_outcomes(&self, market: &MarketType) -> Option<HashSet<String>> {
        let mut outcome_set: HashSet<String> = HashSet::new();

        let specified_market = self.markets.iter().find(|x| x.key == *market);

        let market_outcomes = match specified_market {
            Some(x) => x.outcomes.clone(),
            None => return None,
        };

        let outcome_names = market_outcomes.iter().map(|x| x.name.clone());
        outcome_set.extend(outcome_names);

        return Some(outcome_set);
    }
}
