use bigdecimal::{BigDecimal, ToPrimitive};
use serde::Serialize;

use crate::db::models::{bookies::Book, odds::OddsOffering};

#[derive(Serialize)]
pub struct BookieWithOdds {
    #[serde(flatten)]
    pub bookie: Book,
    pub odds_offerings: Vec<OddsOffering>,
}

impl BookieWithOdds {
    pub fn true_probability_estimate(&self, outcome: &String) -> Option<f64> {
        let odds = match self.get_odds_for_outcome(outcome) {
            None => return None,
            Some(x) => x.to_f64().unwrap(),
        };

        let margin = self.total_probability() - 1.0;
        let n = self.odds_offerings.len() as f64;

        return Some((n - margin * odds) / (n * odds));
    }

    pub fn get_odds_for_outcome(&self, outcome: &String) -> Option<BigDecimal> {
        let maybe_offering = self.find_ofering_for_outcome(outcome);

        if (maybe_offering.is_none()) {
            return None;
        }

        return Some(maybe_offering.unwrap().offered_odds.clone());
    }

    fn find_ofering_for_outcome(&self, outcome: &String) -> Option<&OddsOffering> {
        return self.odds_offerings.iter().find(|x| x.outcome.eq(outcome));
    }

    pub fn get_offering_for_outcome(&self, outcome: &String) -> Option<OddsOffering> {
        return self.find_ofering_for_outcome(outcome).cloned();
    }

    pub fn total_probability(&self) -> f64 {
        return self
            .odds_offerings
            .iter()
            .fold(0.0, |sum, outcome| sum + outcome.implied_probability());
    }
}

// impl Market {
//     pub fn get_vig(&self) -> f64 {
//         let total_probability = self.total_probability();
//         let overround = total_probability - 1.0;
//         return overround / (1.0 + overround);
//     }

//     pub fn get_all_outcomes(&self) -> HashSet<String> {
//         let mut outcomes_set: HashSet<String> = HashSet::new();
//         for outcome in &self.outcomes {
//             let _ = outcomes_set.insert(outcome.name.clone());
//         }
//         return outcomes_set;
//     }

//     pub fn odds_for_outcome(&self, outcome_key: &str) -> Option<Odds> {
//         match self.find_outcome(outcome_key) {
//             Some(outcome) => return Some(outcome.price),
//             None => return None,
//         }
//     }

//     fn find_outcome(&self, outcome_key: &str) -> Option<&Outcome> {
//         for outcome in &self.outcomes {
//             if (outcome.name == outcome_key) {
//                 return Some(outcome);
//             }
//         }
//         return None;
//     }

//     pub fn total_probability(&self) -> f64 {
//         return self
//             .outcomes
//             .iter()
//             .fold(0.0, |sum, outcome| sum + outcome.implied_probability());
//     }

//     pub fn true_probability_for_outcome(&self, outcome_key: &str) -> Option<f64> {
//         match self.find_outcome(outcome_key) {
//             Some(outcome) => return Some(self.true_probability_estimate(&outcome.price)),
//             None => return None,
//         }
//     }

//     // https://cran.r-project.org/web/packages/implied/vignettes/introduction.html
//     pub fn true_probability_estimate(&self, odds: &Odds) -> f64 {
//         let all_odds: Vec<Odds> = self.outcomes.iter().map(|x| x.price.clone()).collect();
//         return odds.true_probability_estimate(&all_odds);
//     }
