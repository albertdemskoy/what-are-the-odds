use std::io;
use odds_interface::api_requests::{get_example_odds_file, get_key_usage, get_odds_for_sport_aus, get_sports};

mod odds_interface;

fn main() {
    let mut num_inputs = 6;
    while num_inputs > 0 {
        println!("Available operations:");
        println!("==========================");
        println!("a:   best odds pair testing");
        println!("s:   print in-season sports");
        println!("o:   odds for a sport of your chosing");

        
        let mut operation_choice = String::new();

        io::stdin()
            .read_line(&mut operation_choice)
            .expect("Failed to read line");
        operation_choice = operation_choice.trim().to_string();

        if operation_choice == "a" {
            let events = get_example_odds_file("./src/soccerodds.json");
            for event in events {
                event.get_best_odds_pair();
            }
        } else if operation_choice == "s" {
            let sports = get_sports().expect("Failed to get sports");
            println!("{sports:#?}")
        }  else if operation_choice == "o" {
            println!("write your sport key of choice");

            let mut sport_key = String::new();
            io::stdin()
            .read_line(&mut sport_key)
            .expect("Failed to read line");
            sport_key = sport_key.trim().to_string();

            let event_raw = get_odds_for_sport_aus(&sport_key).expect("Failed to get odds for");
            println!("{event_raw:?}")
        } else {
            println!("{operation_choice:#?} is not a valid choice!")
        }

        let key_usage = get_key_usage();
        match key_usage {
            Some(x) => println!("requests remaining: {0}", x.requests_remaining),
            None => println!("")
        };

        num_inputs -= 1;
    }
}
