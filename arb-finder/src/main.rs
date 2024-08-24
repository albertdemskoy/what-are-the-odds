use common::MarketType;
use db::{
    establish_connection,
    models::{events::create_event, sports::create_sport},
};
use discover_odds::odds_api::{db::save_odds_to_db, requests::get_odds_for_sport, Region};
use odds_interface::odds_api::{get_events, get_key_usage, get_sports};
use std::io;
use strum::IntoEnumIterator;

mod common;
mod db;
mod discord;
mod discover_odds;
mod local_env;
mod odds_interface;
mod schema;

fn get_trimmed_input() -> String {
    let mut operation_choice = String::new();
    io::stdin()
        .read_line(&mut operation_choice)
        .expect("Failed to read line");

    operation_choice = operation_choice.trim().to_string();
    return operation_choice;
}

fn main() {
    let mut num_inputs = 6;
    while num_inputs > 0 {
        println!("Available operations:");
        println!("==========================");
        println!("s: write in-season sports to db");
        println!("e: write events for sport key to db");
        println!("o: write odds for sport to db, and update bookies");

        let operation_choice = get_trimmed_input();

        if operation_choice == "s" {
            let sports = get_sports().expect("Failed to get sports");
            let connection = &mut establish_connection();
            for sport in sports {
                create_sport(
                    connection,
                    sport.key.as_str(),
                    sport.group.as_str(),
                    sport.title.as_str(),
                );
            }
        } else if operation_choice == "e" {
            let sport_key = get_trimmed_input();
            let sport_events = get_events(&sport_key).expect("Failed to get events");
            let connection = &mut establish_connection();

            for event in sport_events {
                let event = create_event(
                    connection,
                    sport_key.as_str(),
                    event.home_team.as_str(),
                    &event.away_team.as_str(),
                    event.commence_time,
                );
            }
        } else if operation_choice == "o" {
            let sport_key = get_trimmed_input();
            let enabled_markets = [MarketType::H2h, MarketType::Totals];

            let conn = &mut establish_connection();
            for region in Region::iter() {
                let sport_events =
                    get_odds_for_sport(&sport_key, &enabled_markets.to_vec(), &region)
                        .expect("Failed to get events");

                save_odds_to_db(conn, sport_key.as_str(), &sport_events, &region);
            }
        }

        let key_usage = get_key_usage();
        match key_usage {
            Some(x) => println!("requests remaining: {0}", x.requests_remaining),
            None => println!(""),
        };

        num_inputs -= 1;
    }
}
