use core::fmt;
use serde::Deserialize;
use std::collections::HashSet;
use strum_macros::EnumIter;

use super::odds::Odds;

#[derive(Deserialize, Debug, Clone)]
pub struct Market {
    pub key: MarketType,
    pub outcomes: Vec<Outcome>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum MarketType {
    H2h,
    H2hLay,
    Spreads,
    Totals,
    Outrights,
    OutrightsLay,
}

impl fmt::Display for MarketType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MarketType::H2h => write!(f, "h2h"),
            MarketType::H2hLay => write!(f, "h2h_lay"),
            MarketType::Spreads => write!(f, "spreads"),
            MarketType::Totals => write!(f, "totals"),
            MarketType::Outrights => write!(f, "outrights"),
            MarketType::OutrightsLay => write!(f, "outrights_lay"),
        }
    }
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
        let all_odds: Vec<Odds> = self.outcomes.iter().map(|x| x.price.clone()).collect();
        return odds.true_probability_estimate(&all_odds);
    }
}

pub const OVER_OUTCOME: &str = "Over";
pub const UNDER_OUTCOME: &str = "Under";

#[derive(Deserialize, Debug, Clone)]
pub struct Outcome {
    pub name: String,
    pub price: Odds,
    pub point: Option<f64>,
}

impl Outcome {
    fn implied_probability(&self) -> f64 {
        return self.price.implied_probability();
    }

    pub fn set_price(&mut self, new_price: &Odds) {
        self.price = new_price.clone();
    }
}
