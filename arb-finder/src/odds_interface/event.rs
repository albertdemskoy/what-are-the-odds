use serde::Deserialize;
use chrono::{DateTime, Utc};

use crate::odds_interface::event;

use super::odds::{Odds, is_arb};
use super::market::Market;

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

    pub fn get_vigs(&self) {
        for bookmaker in &self.bookmakers {
            for market in &bookmaker.markets {
                if (market.key.contains("lay")) {continue;}

                let vig = market.get_vig();
                println!("Vig of {0} for {1} vs {2} in {3}: {4}", 
                bookmaker.key, 
                self.home_team, 
                self.away_team, 
                market.key, 
                vig);
            }
        }   
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