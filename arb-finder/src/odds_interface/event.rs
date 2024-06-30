use std::collections::HashSet;

use serde::Deserialize;
use chrono::{DateTime, Utc};

use super::odds::{Odds, is_arb};
use super::market::{BookieStat, Bookmaker, Market};

#[derive(Deserialize, Debug, Clone)]
pub struct Event {
    id: String,
    sport_key: String,
    commence_time: DateTime<Utc>,
    home_team: String, 
    away_team: String,
    bookmakers: Vec<Bookmaker>
}

pub fn get_bookie_keys(events: &Vec<Event>) -> HashSet<String> {
    let mut bookie_name_set = HashSet::new();
    for event in events {
        bookie_name_set.extend(event.get_all_bookies())
    }
    return bookie_name_set;
}

pub fn get_average_bookie_vig(events: &Vec<Event>, bookie_name: &str) -> BookieStat {   
    let mut all_markets: Vec<Market> = Vec::new();
    for event in events {
        for bookmaker in &event.bookmakers {
            if (bookmaker.key == bookie_name) {
                all_markets.append(&mut bookmaker.get_enabled_markets());
            }
        }
    }

    let total_vig = all_markets.iter().fold(0.0, |sum, market| sum + market.get_vig());

    return BookieStat {
        key: bookie_name.to_string(),
        vig: total_vig/(all_markets.len() as f64)
    };
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

    pub fn get_all_bookies(&self) -> HashSet<String> {
        let mut bookie_name_set = HashSet::new();
        for bookmaker in &self.bookmakers {
            bookie_name_set.insert(bookmaker.key.to_string());
        }
        return bookie_name_set;
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