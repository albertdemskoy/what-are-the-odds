use odds_interface::api_requests::{get_key_usage, get_odds_for_sport_aus, get_sports};
use std::{fs, io};

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
        println!("o:   odds for a sport of your chosing");
        println!("v:   bookie vigs for event");

        let operation_choice = get_trimmed_input();

        if operation_choice == "s" {
            let sports = get_sports().expect("Failed to get sports");
            println!("{sports:#?}")
        } else if operation_choice == "o" {
            println!("write your sport key of choice");

            let sport_key = get_trimmed_input();
            let event_raw = get_odds_for_sport_aus(&sport_key).expect("Failed to get odds for");
            println!("{event_raw:?}");

            let filename = format!("./src/example_responses/{sport_key}_odds.json");

            fs::write(filename, event_raw);
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
