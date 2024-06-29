use std::io;
use odds_interface::{api_requests::{get_odds_for_sport_aus, get_sports}};

mod odds_interface;

fn main() {
    let mut num_inputs = 6;
    while num_inputs > 0 {
        println!("Available operations:");
        println!("==========================");
        println!("s:   print in-season sports");
        println!("o:   odds for a sport of your chosing");

        
        let mut operation_choice = String::new();

        io::stdin()
            .read_line(&mut operation_choice)
            .expect("Failed to read line");
        operation_choice = operation_choice.trim().to_string();

        if operation_choice == "s" {
            let mut sports = get_sports().expect("Failed to get sports");
            println!("{sports:#?}")
        } else if operation_choice == "o" {
            let events = get_odds_for_sport_aus("./src/sportodds.json");
            for event in events {
                event.get_best_odds_pair();
            }
        } else {
            println!("{operation_choice:#?} is not a valid choice!")
        }

        num_inputs -= 1;
    }
}
