use std::collections::HashSet;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::bookmaker::Bookmaker;
use super::market::MarketType;
use super::odds::Odds;

const MAX_ODDS_CUTOFF: f64 = 10.0;
const PERCENT_EV_CUTOFF: f64 = 2.0;

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    id: String,
    sport_key: String,
    commence_time: DateTime<Utc>,
    home_team: String,
    away_team: String,
    pub bookmakers: Vec<Bookmaker>,
}

pub struct Opportunity<'a> {
    bookie_key: String,
    offered_odds: Odds,
    true_odds: Odds,
    event: &'a Event,
    outcome_key: String,
    market_key: MarketType,
    percent_ev: f64,
}

impl fmt::Display for Opportunity<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let markdown_str = format!(
            "## {0:.2}%: {1} vs {2}
    - Market: {8}
    - {3}: Offering {4:.2} for {5}
    - true odds: {6:.2}
    - commence time: {7}
",
            self.percent_ev,
            self.event.home_team,
            self.event.away_team,
            self.bookie_key,
            self.offered_odds.get_decimal(),
            self.outcome_key,
            self.true_odds.get_decimal(),
            self.event.commence_time,
            self.market_key
        );

        return write!(f, "{}", markdown_str);
    }
}

impl Event {
    pub fn get_all_bookies(&self) -> HashSet<String> {
        let mut bookie_name_set = HashSet::new();
        for bookmaker in &self.bookmakers {
            bookie_name_set.insert(bookmaker.key.to_string());
        }
        return bookie_name_set;
    }

    // TODO: convert to Opportunity struct with more info:
    // - match
    // - market
    // - outcome
    pub fn identify_opportunities(&self, market: &MarketType) -> Vec<Opportunity> {
        let all_outcomes = self.get_all_outcomes(&market);

        let mut opportunities_vec: Vec<Opportunity> = Vec::new();

        for outcome_key in &all_outcomes {
            let true_odds = self.get_true_odds_for_outcome(&market, outcome_key.as_str());

            if (true_odds.get_decimal() > MAX_ODDS_CUTOFF) {
                // only want to consider likely outcomes
                // as odds for unlikely outcomes are skewed
                continue;
            }

            for bookie in &self.bookmakers {
                let maybe_bookie_odds = bookie.get_odds(&market, outcome_key.as_str());

                let bookie_odds = match maybe_bookie_odds {
                    Some(x) => x,
                    None => continue,
                };

                let percent_ev = bookie_odds.ev_percentage(&true_odds);

                if (bookie_odds > true_odds && percent_ev > PERCENT_EV_CUTOFF) {
                    let opportunity = Opportunity {
                        bookie_key: bookie.key.clone(),
                        offered_odds: bookie_odds,
                        outcome_key: outcome_key.clone(),
                        market_key: market.clone(),
                        true_odds,
                        event: self,
                        percent_ev,
                    };

                    opportunities_vec.push(opportunity);
                }
            }
        }

        return opportunities_vec;
    }

    fn get_all_outcomes(&self, market: &MarketType) -> HashSet<String> {
        let mut outcome_set: HashSet<String> = HashSet::new();

        for bookie in &self.bookmakers {
            let bookie_outcomes = bookie.get_offered_outcomes(market);
            match bookie_outcomes {
                Some(x) => outcome_set.extend(x),
                None => continue,
            }
        }

        return outcome_set;
    }

    fn get_true_odds_for_outcome(&self, market: &MarketType, outcome_key: &str) -> Odds {
        let all_bookie_keys = self.get_all_bookies();
        let outcome_avg_probability =
            self.get_average_probability(all_bookie_keys, &market, outcome_key);
        return Odds::Decimal(1.0 / outcome_avg_probability);
    }

    fn get_average_probability(
        &self,
        bookies: HashSet<String>,
        market: &MarketType,
        outcome_key: &str,
    ) -> f64 {
        // sharps vs nonsharps: 50-50 weighting
        return bookies
            .iter()
            .filter(|bookie| {
                self.get_adjusted_probability(bookie, &market, outcome_key)
                    .is_some()
            })
            .fold(0.0, |sum, bookie_key| {
                sum + self
                    .get_adjusted_probability(bookie_key, market, outcome_key)
                    .unwrap()
            })
            / (bookies.len() as f64);
    }

    fn get_adjusted_probability(
        &self,
        bookie_key: &str,
        market: &MarketType,
        outcome: &str,
    ) -> Option<f64> {
        let bookie_object = match self
            .bookmakers
            .iter()
            .find(|bookie| bookie.key == bookie_key)
        {
            Some(x) => x,
            None => return None,
        };

        let markets = &bookie_object.markets;
        let market = match markets.iter().find(|&x| x.key == *market) {
            Some(x) => x,
            None => return None,
        };

        return market.true_probability_for_outcome(outcome);
    }

    fn get_best_odds_for_outcome(&self, market: MarketType, outcome_key: &str) -> (String, Odds) {
        let mut best_odds = 0.0;
        let mut best_bookie_key = String::from("");

        for bookmaker in &self.bookmakers {
            let markets = &bookmaker.markets;
            let market = match markets.iter().find(|&x| x.key == market) {
                Some(x) => x,
                None => continue,
            };

            let team_outcome = match market.outcomes.iter().find(|&x| x.name == outcome_key) {
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
