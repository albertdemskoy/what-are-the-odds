use std::fs;

use reqwest::Error;

use super::response_types::{Event, Sport};
use super::{API_KEY, ODDS_HOST_BASE};

// todo: these should return the actual type
pub fn get_odds_for_sport_aus(sport : &str) -> Vec<Event> {
    // let odds_endpoint = format!("/sports/{sport}/odds/");
    // let full_url =  ODDS_HOST_BASE.to_owned() + &odds_endpoint;

    // let params = [
    //     ("apiKey", API_KEY),
    //     ("regions", "au")
    // ];

    // let url = reqwest::Url::parse_with_params(&full_url, &params).unwrap();
    // let res = match reqwest::blocking::get(url) {
    //     Ok(x) => x,
    //     Err(e) => return Err(e)
    // };

    // return Ok(res.json::<Vec<Event>>().unwrap());

    let file_str = fs::read_to_string("../exampleresponses/sportodds.json").expect("Unable to read file");
    return serde_json::from_str::<Vec<Event>>(&file_str).expect("JSON was not well-formatted");
}

pub fn get_sports() -> Result<Vec<Sport>, Error> {
    let sports_endpoint = "/sports/";
    let full_url =  ODDS_HOST_BASE.to_owned() + sports_endpoint;
    let params = [
        ("apiKey", API_KEY),
    ];

    let url = reqwest::Url::parse_with_params(&full_url, &params).unwrap();
    let res = match reqwest::blocking::get(url) {
        Ok(x) => x,
        Err(e) => return Err(e)
   };

    let body = res.json::<Vec<Sport>>().unwrap_or(Vec::new());

    return Ok(body);
}