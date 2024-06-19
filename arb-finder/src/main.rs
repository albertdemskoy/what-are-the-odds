use std::io;
use odds_interface::api_requests::get_sports;

mod odds_interface;

fn main() {
    let mut num_inputs = 6;
    while num_inputs > 0 {
        println!("Available operations:");
        println!("==========================");
        println!("s:   print in-season sports");
        println!("o:   odds for a sport of your chosing");

        
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        choice = choice.trim().to_string();

        if choice == "s" {
            let sports = get_sports().expect("Failed to get sports");
            println!("{sports:#?}")
        } else {
            println!("{choice:#?} is not a valid choice!")
        }

        num_inputs -= 1;
    }
}
