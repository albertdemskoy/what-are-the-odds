use std::collections::HashSet;

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

    pub fn get_offered_outcomes(&self) -> HashSet<String> {
        let mut outcome_set: HashSet<String> = HashSet::new();

        for market in &self.markets {
            outcome_set.extend(market.get_all_outcomes())
        }

        return outcome_set;
    }
}
