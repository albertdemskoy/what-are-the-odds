use serde::Deserialize;
use std::collections::HashSet;

use super::odds::Odds;

#[derive(Deserialize, Debug, Clone)]
pub struct Market {
    pub key: MarketType,
    pub outcomes: Vec<Outcome>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MarketType {
    H2h,
    H2hLay,
    Spreads,
    Totals,
    Outrights,
    OutrightsLay,
}

impl Market {
    pub fn get_vig(&self) -> f64 {
        let total_probability = self.total_probability();
        let overround = total_probability - 1.0;
        return overround / (1.0 + overround);
    }

    pub fn get_all_outcomes(&self) -> HashSet<String> {
        let mut outcomes_set: HashSet<String> = HashSet::new();
        for outcome in &self.outcomes {
            let _ = outcomes_set.insert(outcome.name.clone());
        }
        return outcomes_set;
    }

    pub fn odds_for_outcome(&self, outcome_key: &str) -> Option<Odds> {
        match self.find_outcome(outcome_key) {
            Some(outcome) => return Some(outcome.price),
            None => return None,
        }
    }

    fn find_outcome(&self, outcome_key: &str) -> Option<&Outcome> {
        for outcome in &self.outcomes {
            if (outcome.name == outcome_key) {
                return Some(outcome);
            }
        }
        return None;
    }

    pub fn total_probability(&self) -> f64 {
        return self
            .outcomes
            .iter()
            .fold(0.0, |sum, outcome| sum + outcome.implied_probability());
    }

    pub fn true_probability_for_outcome(&self, outcome_key: &str) -> Option<f64> {
        match self.find_outcome(outcome_key) {
            Some(outcome) => return Some(self.true_probability_estimate(&outcome.price)),
            None => return None,
        }
    }

    // https://cran.r-project.org/web/packages/implied/vignettes/introduction.html
    pub fn true_probability_estimate(&self, odds: &Odds) -> f64 {
        let margin = self.total_probability() - 1.0;
        let n = self.outcomes.len() as f64;
        let raw_odds = odds.get_decimal();

        return (n - margin * raw_odds) / (n * raw_odds);
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Outcome {
    pub name: String,
    pub price: Odds,
}

impl Outcome {
    fn implied_probability(&self) -> f64 {
        return self.price.implied_probability();
    }
}
