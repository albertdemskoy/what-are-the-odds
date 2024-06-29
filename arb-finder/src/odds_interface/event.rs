use serde::Deserialize;
use chrono::{DateTime, Utc};

use super::odds::{Odds, is_arb};

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    id: String,
    sport_key: String,
    commence_time: DateTime<Utc>,
    home_team: String, 
    away_team: String,
    bookmakers: Vec<Bookmaker>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Bookmaker {
    key: String,
    title: String,
    last_update: DateTime<Utc>,
    markets: Vec<Market>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Market {
    key: String,
    outcomes: Vec<Outcome>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Outcome {
    name: String,
    price: Odds,
}

impl Event {
    pub fn get_best_odds_pair(&self) {
        // get best odds for home_team
        let home_team = &self.home_team;
        let (best_home_bookie, best_home_odds) = self.get_best_odds_for_team(&home_team);
        let away_team = &self.away_team;
        let (best_away_bookie, best_away_odds) = self.get_best_odds_for_team(&away_team);

        println!("{0}: {1} at {2} from {3}, vs {4} at {5} from {6}", 
            self.sport_key, 
            home_team, 
            best_home_odds.get_decimal(), 
            best_home_bookie, 
            away_team, 
            best_away_odds.get_decimal(), 
            best_away_bookie);

        println!("is arb? {0}", is_arb(best_home_odds, best_away_odds));
    }

    fn get_best_odds_for_team(&self, team_name: &str) -> (String, Odds) {
        let h2h_market_key = "h2h";

        let mut best_odds = 0.0;
        let mut best_bookie_key = String::from("");

        for bookmaker in &self.bookmakers {
            let markets = &bookmaker.markets;
            let h2h_market = match markets.iter().find(|&x| x.key == h2h_market_key) {
                Some(x) => x,
                None => continue
            };

            let team_outcome = match h2h_market.outcomes.iter().find(|&x| x.name == team_name) {
                Some(x) => x,
                None => continue
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