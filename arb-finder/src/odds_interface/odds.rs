use std::{cmp::{self, Ordering}, ops};
use serde::Deserialize;

fn american_to_decimal(american_odds: i32) -> f64 {
    let f_american_odds = f64::from(american_odds);

    if (f_american_odds > 0.0) {
        // positive: american odds = profit off 100
        let total_return = 100.0 + f_american_odds;
        return total_return/100.0;
    } else {
        // negative: american odds = $ bet required to make 100 profit
        let initial = -f_american_odds;
        let total_return = 100.0 + initial;
        return total_return/initial;
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(untagged)]
pub enum Odds {
    American(i32),
    Decimal(f64)
}

impl Odds {
    pub fn as_decimal(self) -> Self {
        return Odds::Decimal(self.get_decimal());
    } 

    pub fn get_decimal(&self) -> f64 {
        return match self {
            Odds::American(x) => american_to_decimal(*x),
            Odds::Decimal(x) => *x
        }        
    }
}

impl ops::Add<Odds> for Odds{
    type Output = Odds;
    fn add(self, _rhs: Self) -> Self {
        return Odds::Decimal(self.get_decimal() + _rhs.get_decimal());
    }
}

impl ops::Mul<Odds> for Odds{
    type Output = Odds;
    fn mul(self, _rhs: Self) -> Self {
        return Odds::Decimal(self.get_decimal() * &_rhs.get_decimal());
    }
}

impl cmp::PartialOrd for Odds {
    fn partial_cmp(&self, _rhs: &Self) -> Option<Ordering> {
        return self.get_decimal().partial_cmp(&_rhs.get_decimal());
    }
} 

/// # Params: decimal odds for two mutually exclusive events
/// 
pub fn is_arb(odds1: Odds, odds2: Odds) -> bool {
    let odds1_dec = odds1.as_decimal();
    let odds2_dec = odds2.as_decimal();
    
    let result = odds1_dec * odds2_dec > odds1_dec + odds2_dec;
    return result;
}