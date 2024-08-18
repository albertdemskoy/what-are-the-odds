use db::{establish_connection, models::create_sport};
use odds_interface::odds_api::{get_key_usage, get_sports};
use std::io;

mod db;
mod local_env;
mod messaging;
mod odds_interface;
mod schema;

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
        println!("s: write in-season sports to db");

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
        }

        let key_usage = get_key_usage();
        match key_usage {
            Some(x) => println!("requests remaining: {0}", x.requests_remaining),
            None => println!(""),
        };

        num_inputs -= 1;
    }
}
