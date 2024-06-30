use std::{fs, io};
use odds_interface::{api_requests::{get_example_odds_file, get_key_usage, get_odds_for_sport_aus, get_sports}, event::{get_average_bookie_vig, get_bookie_keys}};

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

        
        let mut operation_choice = get_trimmed_input();

        if operation_choice == "s" {
            let sports = get_sports().expect("Failed to get sports");
            println!("{sports:#?}")
        }  else if operation_choice == "o" {
            println!("write your sport key of choice");

            let sport_key = get_trimmed_input();
            let event_raw = get_odds_for_sport_aus(&sport_key).expect("Failed to get odds for");
            println!("{event_raw:?}");

            let filename = format!("./src/example_responses/{sport_key}_odds.json");

            fs::write(filename, event_raw);
        } else if operation_choice == "v" {
            println!("Enter sport key:");
            let sport_key = get_trimmed_input();

            let filename = get_sport_key_json(&sport_key);
            let events = get_example_odds_file(&filename);
            
            let bookie_names = get_bookie_keys(&events);
            let mut bookie_stats = Vec::new();
            for bookie_name in bookie_names {
                let avg_vig = get_average_bookie_vig(&events, &bookie_name);
                bookie_stats.push(avg_vig);
            }

            bookie_stats.sort_by(|a,b| a.vig.total_cmp(&b.vig));
            let mut i = 1;
            for bookie_stat in bookie_stats {
                println!("{0}st place: {1} with vig {2}", i, bookie_stat.key, bookie_stat.vig);

                i += 1;
            }

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
