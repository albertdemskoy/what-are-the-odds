use bigdecimal::{BigDecimal, ToPrimitive};

use crate::db::models::odds::OddsOffering;

use super::{bookie_odds::BookieWithOdds, util::calculate_ev_percentage, MarketType};

pub struct Outcome {
    pub name: String,
    pub line: Option<BigDecimal>,
}

pub struct MarketInfo {
    pub all_bookies_offerings: Vec<BookieWithOdds>,
    pub market_type: MarketType,
    pub all_outcomes: Vec<String>,
}

pub struct OpportunityConfig {
    pub minimum_ev: i32,
}

pub struct Opportunity {
    true_prob: f64,
    offering: OddsOffering,
}

impl MarketInfo {
    fn get_opportunities(&self, config: OpportunityConfig) -> Vec<Opportunity> {
        let mut opps_vec = Vec::new();

        if self.market_type == MarketType::H2h {
            for outcome in &self.all_outcomes {
                let true_prob = self.true_probability_estimate(outcome);

                for bookie_offerings in &self.all_bookies_offerings {
                    let offered_odds = match bookie_offerings.get_odds_for_outcome(outcome) {
                        None => continue,
                        Some(x) => x,
                    };

                    if (calculate_ev_percentage(offered_odds.to_f64().unwrap(), true_prob)
                        > config.minimum_ev as f64)
                    {
                        opps_vec.push(Opportunity {
                            true_prob,
                            offering: bookie_offerings.get_offering_for_outcome(outcome).unwrap(),
                        })
                    }
                }
            }
        }

        return opps_vec;
    }

    fn get_true_odds_estimates(&self, outcome: &String) -> Vec<f64> {
        return self
            .all_bookies_offerings
            .iter()
            .map(|x| x.true_probability_estimate(outcome))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
    }

    pub fn true_probability_estimate(&self, outcome: &String) -> f64 {
        if (self.market_type == MarketType::H2h) {
            let with_outcome = self.get_true_odds_estimates(outcome);
            let total_prob: f64 = with_outcome.iter().sum();
            let n = with_outcome.len();

            return total_prob / n as f64;
        }

        let panic_output = format!("Market type {0} not supported!!", self.market_type);
        panic!("{}", panic_output);
    }
}
