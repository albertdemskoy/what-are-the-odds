use messaging::send_message;
use odds_interface::{
    api_requests::{get_key_usage, get_odds_for_sport, get_sports},
    bookmaker::Region,
    market::MarketType,
};
use std::io;

mod local_env;
mod messaging;
mod odds_interface;

fn get_sport_key_json(sport_key: &str) -> String {
    return format!("./src/example_responses/{sport_key}_odds.json");
}

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
        println!("s:   print in-season sports");
        println!("e:   try to find +EV opportunities for sport of choosing");
        println!("m:   send test message to discord server");

        let operation_choice = get_trimmed_input();

        if operation_choice == "s" {
            let sports = get_sports().expect("Failed to get sports");
            println!("{sports:#?}")
        } else if operation_choice == "e" {
            println!("write your sport key of choice");

            let sport_key = get_trimmed_input();
            let markets = [MarketType::H2h, MarketType::Spreads, MarketType::Totals].to_vec();
            let regions = [Region::Us, Region::Uk, Region::Au, Region::Eu].to_vec();

            let events_raw = get_odds_for_sport(&sport_key, &markets, &regions)
                .expect("Failed to get odds for {sport_key:?}");
            for event in events_raw {
                let opportunities = event.identify_opportunities(MarketType::H2h);
                for opportunity in opportunities {
                    send_message(&opportunity);
                }
            }
        } else if operation_choice == "m" {
            println!("sending test message to discord server");
            send_message("Hello World!!")
        } else {
            println!("{operation_choice:#?} is not a valid choice!")
        }

        let key_usage = get_key_usage();
        match key_usage {
            Some(x) => println!("requests remaining: {0}", x.requests_remaining),
            None => println!(""),
        };

        num_inputs -= 1;
    }
}
