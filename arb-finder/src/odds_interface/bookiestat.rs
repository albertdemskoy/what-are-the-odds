use std::collections::HashSet;

use super::{event::Event, market::Market};

pub struct BookieStat {
    pub key: String,
    pub vig: f64
}

pub fn get_bookie_keys(events: &Vec<Event>) -> HashSet<String> {
    let mut bookie_name_set = HashSet::new();
    for event in events {
        bookie_name_set.extend(event.get_all_bookies())
    }
    return bookie_name_set;
}


pub fn sharp_book_keys() -> Vec<String> {
    // to_do: need to filter exchanges based on how active they are ... i.e. if vig  < 0.05
    return ["pinnacle"]
        .iter()
        .map(|x| x.to_string())
        .collect();
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

