use std::collections::HashSet;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use statrs::distribution::{ContinuousCDF, Normal};

use super::market::{OVER_OUTCOME, UNDER_OUTCOME};

mod event_test;

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
    bookmakers: Vec<Bookmaker>,
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

    fn lambda_estimate(cumulative_prob: f64, offered_line: f64) -> f64 {
        //TODO: Document
        let standard_normal = Normal::standard();

        let b =
            2.0 * offered_line + Normal::inverse_cdf(&standard_normal, cumulative_prob).powf(2.0);
        let term2 = b.powf(2.0) - 4.0 * offered_line.powf(2.0);

        return (b + term2.sqrt()) / 2.0;
    }

    fn identify_totals_opportunities(&self) -> Vec<Opportunity> {
        let mut opps: Vec<Opportunity> = Vec::new();
        const MARKET_KEY: MarketType = MarketType::Totals;

        // SHOULD USE A POISSON DEPENDING ON LINE MAGNITUDE
        // estimate the true score distribution

        let mut lamb_estimates: Vec<f64> = Vec::new();
        for bookie in &self.bookmakers {
            let outcomes = bookie.get_offered_outcomes(&MARKET_KEY);
            let both_sides_odds: Vec<Odds> = outcomes.iter().map(|x| x.price).collect();
            let under_outcome = outcomes.iter().find(|x| x.name == UNDER_OUTCOME).unwrap();

            let price = under_outcome.price;
            let point = under_outcome.point.unwrap();

            let implied_prob = price.true_probability_estimate(&both_sides_odds);

            let lamb_estimate = Self::lambda_estimate(implied_prob, point);
            lamb_estimates.push(lamb_estimate);
        }

        let lamb_avg =
            lamb_estimates.iter().fold(0.0, |acc, x| acc + x) / (lamb_estimates.len() as f64);

        for bookie in &self.bookmakers {
            let outcomes = bookie.get_offered_outcomes(&MARKET_KEY);
            for outcome in outcomes {
                let offered_line = outcome.point.unwrap();

                let bookie_odds = outcome.price;

                // get true odds of this line
                let mut true_probability =
                    Normal::standard().cdf((offered_line - lamb_avg) / lamb_avg.sqrt());
                if (outcome.name == OVER_OUTCOME) {
                    true_probability = 1.0 - true_probability;
                }

                let true_odds = Odds::Decimal(1.0 / true_probability);

                let percent_ev = bookie_odds.ev_percentage(&true_odds);

                if (bookie_odds > true_odds && percent_ev > PERCENT_EV_CUTOFF) {
                    let opportunity = Opportunity {
                        bookie_key: bookie.key.clone(),
                        offered_odds: bookie_odds,
                        outcome_key: outcome.name.clone(),
                        market_key: MARKET_KEY.clone(),
                        true_odds,
                        event: self,
                        percent_ev,
                    };

                    opps.push(opportunity);
                }
            }
        }

        return opps;
    }

    fn identify_h2h_opportunities(&self) -> Vec<Opportunity> {
        const MARKET_KEY: MarketType = MarketType::H2h;
        let all_outcomes = self.get_all_outcomes(&MARKET_KEY);

        let mut opportunities_vec: Vec<Opportunity> = Vec::new();

        for outcome_key in &all_outcomes {
            let true_odds = self.get_true_odds_for_outcome(&MARKET_KEY, outcome_key.as_str());

            if (true_odds.get_decimal() > MAX_ODDS_CUTOFF) {
                // only want to consider likely outcomes
                // as odds for unlikely outcomes are skewed
                continue;
            }

            for bookie in &self.bookmakers {
                let maybe_bookie_odds = bookie.get_odds(&MARKET_KEY, outcome_key.as_str());

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
                        market_key: MARKET_KEY.clone(),
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

    pub fn identify_opportunities(&self, market: &MarketType) -> Vec<Opportunity> {
        if (*market == MarketType::H2h) {
            return self.identify_h2h_opportunities();
        } else if (*market == MarketType::Totals) {
            return self.identify_totals_opportunities();
        }

        return Vec::new();
        // for over under we want to do it differently:
        // - different lines with different odds
        // - find the estimate for true score distribution
        // - go through the offerings and see if there is anything that isn't up to par
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
}
