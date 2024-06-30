use std::collections::HashSet;

use serde::Deserialize;
use chrono::{DateTime, Utc};

use super::odds::Odds;
use super::market::{Bookmaker};

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    id: String,
    sport_key: String,
    commence_time: DateTime<Utc>,
    home_team: String, 
    away_team: String,
    pub bookmakers: Vec<Bookmaker>
}

impl Event {
    pub fn get_all_bookies(&self) -> HashSet<String> {
        let mut bookie_name_set = HashSet::new();
        for bookmaker in &self.bookmakers {
            bookie_name_set.insert(bookmaker.key.to_string());
        }
        return bookie_name_set;
    }

    fn get_average_probability(&self, bookies: HashSet<String>, outcome_key: &str) -> f64{
        // sharps vs nonsharps: 50-50 weighting
        return bookies
            .iter()
            .filter(|bookie| self.get_bookie_probability(bookie, outcome_key).is_some())
            .fold(0.0, |sum, bookie_key| sum + self.get_bookie_probability(bookie_key, outcome_key).unwrap())
            /(bookies.len() as f64);
    }


    fn get_bookie_probability(&self, bookie_key: &str, outcome: &str) -> Option<f64> {
        let bookie_object = match self.bookmakers.iter().find(|bookie| bookie.key == bookie_key) {
            Some(x) => x,
            None => return None
        };

        let markets = &bookie_object.markets;
        let h2h_market_key = "h2h";
        let h2h_market = match markets.iter().find(|&x| x.key == h2h_market_key) {
            Some(x) => x,
            None => return None
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
                None => continue
            };

            let team_outcome = match h2h_market.outcomes.iter().find(|&x| x.name == outcome_key) {
                Some(x) => x,
                None => continue
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