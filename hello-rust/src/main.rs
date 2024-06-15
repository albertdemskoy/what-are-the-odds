fn get_sports() {

    let ODDS_HOST_BASE = "https://api.the-odds-api.com"

    let response = match reqwest::blocking::get("https://www.rust-lang.org") {
        Ok(res) => res,
        Err(e) => return,
    };
}

fn main() {

    let response = match reqwest::blocking::get("https://www.rust-lang.org") {
        Ok(res) => res,
        Err(e) => return,
    };

    let body = response.text();

    println!("body = {body:?}");    
}
