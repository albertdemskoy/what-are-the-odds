use serde::Deserialize;
use super::odds::Odds;

#[derive(Deserialize, Debug, Clone)]
pub struct Market {
    pub key: String,
    pub outcomes: Vec<Outcome>,
}

impl Market {
    pub fn get_vig(&self) -> f64 {
        let total_probability = self.total_probability();
        let overround = total_probability - 1.0;
        return overround/(1.0 + overround);
    }

    pub fn total_probability(&self) -> f64 {
        return self.outcomes.iter().fold(0.0, |sum, outcome| sum + outcome.implied_probability());
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