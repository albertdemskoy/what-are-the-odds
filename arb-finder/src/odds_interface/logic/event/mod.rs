use core::num;
use std::collections::{HashMap, HashSet};
use std::fmt;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use statrs::distribution::{ContinuousCDF, DiscreteCDF, Normal, Poisson};
use strum::IntoEnumIterator;

use super::market::{OVER_OUTCOME, UNDER_OUTCOME};

mod event_test;

use super::bookmaker::Bookmaker;
use super::market::MarketType;
use super::odds::Odds;
use super::{AUS_ONLY, AU_BOOKS};

// TODO: pass these as parameters
const MAX_ODDS_CUTOFF: f64 = 10.0;
const PERCENT_EV_CUTOFF: f64 = 5.0;

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

pub struct Opportunity {
    bookie_name: String,
    offered_odds: Odds,
    sport_title: String,
    home_team: String,
    away_team: String,
    true_odds: Odds,
    message: String,
    outcome_key: String,
    market_key: MarketType,
    percent_ev: f64,
}

impl fmt::Display for Opportunity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let markdown_str = format!(
            " ### Opportunity found on {0}
    - {1:.2}%: {2} vs {3} at {4}
    - Outcome: {5}
    - Market: {6}
    - Offered odds: {7}
    - True odds: {8}
    - Explanation: {9}
",
            self.sport_title,
            self.percent_ev,
            self.home_team,
            self.away_team,
            self.bookie_name,
            self.outcome_key,
            self.market_key,
            self.offered_odds.get_decimal(),
            self.true_odds.get_decimal(),
            self.message,
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

    fn poisson_rate_estimate(cumulative_prob: f64, offered_line: f64) -> f64 {
        let standard_normal = Normal::standard();
        // TODO: continuity correction -- not needed
        // apparently Wilson-Hilferty is a better approximation
        let b =
            2.0 * offered_line + Normal::inverse_cdf(&standard_normal, cumulative_prob).powf(2.0);
        let term2 = b.powf(2.0) - 4.0 * offered_line.powf(2.0);

        return (b + term2.sqrt()) / 2.0;
    }

    fn implied_mean_score(&self, bookie: &Bookmaker) -> Option<f64> {
        let outcomes = bookie.get_offered_outcomes(&MarketType::Totals);

        let both_sides_odds: Vec<Odds> = outcomes.iter().map(|x| x.price).collect();
        let under_outcome_option = outcomes.iter().find(|x| x.name == UNDER_OUTCOME);

        let under_outcome = match under_outcome_option {
            Some(x) => x,
            None => return None,
        };

        let price = under_outcome.price;
        let point = under_outcome.point.unwrap();

        let implied_prob = price.true_probability_estimate(&both_sides_odds);

        let lamb_estimate = Self::poisson_rate_estimate(implied_prob, point);
        return Some(lamb_estimate);
    }

    fn get_relevant_bookies(&self) -> Vec<Bookmaker> {
        if (!AUS_ONLY) {
            return self.bookmakers.clone();
        }

        return self
            .bookmakers
            .clone()
            .into_iter()
            .filter(|x| AU_BOOKS.contains(&x.key.as_str()))
            .collect();
    }

    fn identify_totals_opportunities(&self) -> Vec<Opportunity> {
        let mut opps: Vec<Opportunity> = Vec::new();

        let mut lamb_estimates_for_bookies = HashMap::new();

        let mut lines_set: HashSet<i32> = HashSet::new();
        for bookie in &self.bookmakers {
            let maybe_line = bookie.get_over_under_line();
            if let Some(line) = maybe_line {
                lines_set.insert(line as i32);
            }
        }

        // if every line is the same then no need to worry
        if (lines_set.len() < 2) {
            return self.identify_opportunities_naive(&MarketType::Totals);
        }

        for bookie in &self.bookmakers {
            let lamb_estimate = self.implied_mean_score(bookie);
            match lamb_estimate {
                None => continue,
                Some(x) => lamb_estimates_for_bookies.insert(bookie.key.clone(), x),
            };
        }

        let sum_lambda = lamb_estimates_for_bookies
            .iter()
            .fold(0.0, |acc, (bookie_key, rate_estimate)| acc + rate_estimate);
        let bookies_offering_totals: Vec<String> =
            lamb_estimates_for_bookies.keys().cloned().collect();
        let num_bookies_offering = bookies_offering_totals.len();
        let avg_lambda = sum_lambda as f64 / num_bookies_offering as f64;

        if (avg_lambda <= 0.0 || num_bookies_offering <= 1) {
            return Vec::new();
        }

        let event_au_books: Vec<Bookmaker> = self.get_relevant_bookies();

        for bookie in event_au_books {
            if (!bookies_offering_totals.contains(&bookie.key)) {
                continue;
            }

            // Need to address all the unwrapping here
            let outcomes = bookie.get_offered_outcomes(&MarketType::Totals);
            for outcome in outcomes {
                let offered_line = outcome.point.unwrap();
                let bookie_odds = outcome.price;

                // get true odds of this line -- use normal approximation to estimate lambda
                // but use poisson cdf to calculate
                let poisson_dist = Poisson::new(avg_lambda).unwrap();

                // round down for under probability
                let line_rounded = offered_line as u64;
                let mut true_probability = poisson_dist.cdf(line_rounded);

                if (outcome.name == OVER_OUTCOME) {
                    true_probability = 1.0 - true_probability;
                }

                let true_odds = Odds::Decimal(1.0 / true_probability);

                let percent_ev = bookie_odds.ev_percentage(&true_odds);

                if (percent_ev > PERCENT_EV_CUTOFF) {
                    let opportunity = Opportunity {
                        bookie_name: bookie.title.clone(),
                        offered_odds: bookie_odds,
                        outcome_key: outcome.name.clone(),
                        market_key: MarketType::Totals.clone(),
                        true_odds,
                        percent_ev,
                        sport_title: self.sport_title.clone(),
                        home_team: self.home_team.clone(),
                        away_team: self.away_team.clone(),
                        message: format!(
                            "True line at {0}. Offered line {1}",
                            avg_lambda, offered_line
                        ),
                    };

                    opps.push(opportunity);
                }
            }
        }

        return opps;
    }

    fn identify_opportunities_naive(&self, market: &MarketType) -> Vec<Opportunity> {
        let all_outcomes = self.get_all_outcomes(market);

        let mut opportunities_vec: Vec<Opportunity> = Vec::new();

        for outcome_key in &all_outcomes {
            let true_odds = self.get_true_odds_for_outcome(market, outcome_key.as_str());

            if (true_odds.get_decimal() > MAX_ODDS_CUTOFF) {
                // only want to consider likely outcomes
                // as odds for unlikely outcomes are skewed
                continue;
            }

            let event_au_books: Vec<Bookmaker> = self.get_relevant_bookies();

            for bookie in event_au_books {
                let maybe_bookie_odds = bookie.get_odds(market, outcome_key.as_str());

                let bookie_odds = match maybe_bookie_odds {
                    Some(x) => x,
                    None => continue,
                };

                let percent_ev = bookie_odds.ev_percentage(&true_odds);

                if (bookie_odds > true_odds && percent_ev > PERCENT_EV_CUTOFF) {
                    let opportunity = Opportunity {
                        bookie_name: bookie.title.clone(),
                        sport_title: self.sport_title.clone(),
                        home_team: self.home_team.clone(),
                        away_team: self.away_team.clone(),
                        offered_odds: bookie_odds,
                        outcome_key: outcome_key.clone(),
                        market_key: market.clone(),
                        message: format!("Hello"),
                        true_odds,
                        percent_ev,
                    };

                    opportunities_vec.push(opportunity);
                }
            }
        }
        return opportunities_vec;
    }

    fn identify_h2h_opportunities(&self) -> Vec<Opportunity> {
        const MARKET_KEY: MarketType = MarketType::H2h;
        return self.identify_opportunities_naive(&MARKET_KEY);
    }

    pub fn identify_opportunities_in_market(&self, market: &MarketType) -> Vec<Opportunity> {
        if (*market == MarketType::H2h) {
            return self.identify_h2h_opportunities();
        } else if (*market == MarketType::Totals) {
            return self.identify_totals_opportunities();
        }

        return Vec::new();
    }

    pub fn identify_opportunities(&self) -> Vec<Opportunity> {
        let mut all_opportunities = Vec::new();
        for market_type in MarketType::iter() {
            let mut market_opps = self.identify_opportunities_in_market(&market_type);
            all_opportunities.append(&mut market_opps);
        }
        return all_opportunities;
    }

    fn get_all_outcomes(&self, market: &MarketType) -> HashSet<String> {
        let mut outcome_set: HashSet<String> = HashSet::new();

        for bookie in &self.bookmakers {
            let bookie_outcomes = bookie.get_offered_outcomes(market);
            outcome_set.extend(bookie_outcomes.iter().map(|x| x.name.clone()));
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
                self.get_adjusted_probability(bookie, market, outcome_key)
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
}
