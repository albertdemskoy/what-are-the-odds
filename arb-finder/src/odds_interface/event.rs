use std::collections::HashSet;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::market::{Bookmaker, Market, MarketType};
use super::odds::Odds;

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    id: String,
    sport_key: String,
    commence_time: DateTime<Utc>,
    home_team: String,
    away_team: String,
    pub bookmakers: Vec<Bookmaker>,
}

impl Event {
    pub fn get_all_bookies(&self) -> HashSet<String> {
        let mut bookie_name_set = HashSet::new();
        for bookmaker in &self.bookmakers {
            bookie_name_set.insert(bookmaker.key.to_string());
        }
        return bookie_name_set;
    }

    fn identify_opportunities(&self, market: MarketType) {
        let all_outcomes = self.get_all_outcomes();
        for bookie in self.bookmakers {
            for outcome_key in all_outcomes {
                let bookie_odds = bookie.get_odds(market, outcome_key);
                let true_odds = self.get_true_odds_for_outcome(market, outcome_key);
                if (bookie_odds > true_odds) {
                    println!(
                        "found one!! {0} offering {1} for {2} when true odds are {3}",
                        bookie.key, bookie_odds, outcome_key, true_odds
                    );
                }
            }
        }
    }

    fn get_bookie_odds(&self, bookie_key: &str, market: MarketType, outcome_key: &str) -> f64 {}

    fn get_true_odds_for_outcome(&self, market: MarketType, outcome_key: &str) -> f64 {
        let all_bookie_keys = self.get_all_bookies();
        let outcome_avg_probability = self.get_average_probability(all_bookie_keys, outcome_key);
        return 1.0 / outcome_avg_probability;
    }

    fn get_average_probability(&self, bookies: HashSet<String>, outcome_key: &str) -> f64 {
        // sharps vs nonsharps: 50-50 weighting
        return bookies
            .iter()
            .filter(|bookie| self.get_adjusted_probability(bookie, outcome_key).is_some())
            .fold(0.0, |sum, bookie_key| {
                sum + self
                    .get_adjusted_probability(bookie_key, outcome_key)
                    .unwrap()
            })
            / (bookies.len() as f64);
    }

    fn get_adjusted_probability(&self, bookie_key: &str, outcome: &str) -> Option<f64> {
        let bookie_object = match self
            .bookmakers
            .iter()
            .find(|bookie| bookie.key == bookie_key)
        {
            Some(x) => x,
            None => return None,
        };

        let markets = &bookie_object.markets;
        let h2h_market_key = "h2h";
        let h2h_market = match markets.iter().find(|&x| x.key == h2h_market_key) {
            Some(x) => x,
            None => return None,
        };

        return h2h_market.true_probability_for_outcome(outcome);
    }

    fn get_best_odds_for_outcome(&self, outcome_key: &str) -> (String, Odds) {
        let h2h_market_key = "h2h";

        let mut best_odds = 0.0;
        let mut best_bookie_key = String::from("");

        for bookmaker in &self.bookmakers {
            let markets = &bookmaker.markets;
            let h2h_market = match markets.iter().find(|&x| x.key == h2h_market_key) {
                Some(x) => x,
                None => continue,
            };

            let team_outcome = match h2h_market.outcomes.iter().find(|&x| x.name == outcome_key) {
                Some(x) => x,
                None => continue,
            };

            let bookie_odds = team_outcome.price.get_decimal();
            if bookie_odds > best_odds {
                best_odds = bookie_odds;
                best_bookie_key = bookmaker.key.clone();
            }
        }

        return (best_bookie_key, Odds::Decimal(best_odds));
    }
}
