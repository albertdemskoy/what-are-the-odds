use std::{collections::HashSet, hash::Hash};

use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::odds::Odds;

#[derive(Deserialize, Debug, Clone)]
pub struct Bookmaker {
    pub key: String,
    title: String,
    last_update: DateTime<Utc>,
    pub markets: Vec<Market>,
}

impl Bookmaker {
    pub fn get_enabled_markets(&self) -> Vec<Market> {
        let to_exclude = "lay".to_string();
        return self
            .markets
            .clone()
            .into_iter()
            .filter(|x| !x.key.contains(&to_exclude))
            .collect();
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Market {
    pub key: MarketType,
    pub outcomes: Vec<Outcome>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MarketType {
    H2h,
    Spreads,
    Totals,
    Outrights,
}

impl Market {
    pub fn get_vig(&self) -> f64 {
        let total_probability = self.total_probability();
        let overround = total_probability - 1.0;
        return overround / (1.0 + overround);
    }

    pub fn get_all_outcomes(&self) -> HashSet<String> {
        let mut outcomes_set: HashSet<String> = HashSet::new();
        self.outcomes
            .iter()
            .map(|outcome| outcomes_set.insert(outcome.name.clone()));
        return outcomes_set;
    }

    pub fn total_probability(&self) -> f64 {
        return self
            .outcomes
            .iter()
            .fold(0.0, |sum, outcome| sum + outcome.implied_probability());
    }

    pub fn true_probability_for_outcome(&self, outcome_key: &str) -> Option<f64> {
        let found_outcome = self
            .outcomes
            .iter()
            .find(|outcome| (*outcome).name == outcome_key);

        let outcome = match found_outcome {
            Some(x) => x,
            None => return None,
        };

        let outcome_odds = outcome.price;
        return Some(self.true_probability_estimate(&outcome_odds));
    }

    // https://cran.r-project.org/web/packages/implied/vignettes/introduction.html
    pub fn true_probability_estimate(&self, odds: &Odds) -> f64 {
        let margin = self.total_probability() - 1.0;
        let n = self.outcomes.len() as f64;
        let raw_odds = odds.get_decimal();

        return (n - margin * raw_odds) / (n * raw_odds);
    }

    pub fn true_probability_estimates(&self) -> Vec<f64> {
        let margin = self.total_probability() - 1.0;
        let n = self.outcomes.len() as f64;
        return self
            .outcomes
            .iter()
            .map(|outcome| self.true_probability_estimate(&outcome.price))
            .collect();
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
