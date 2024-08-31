use super::util::{get_key_usage_from_headers, ApiKeyUsage};
use super::{Event, MarketType, Region, Sport};

use reqwest::{blocking::Response, Error};

use crate::local_env::MY_ENV;

const ODDS_HOST_BASE: &str = "https://api.the-odds-api.com/v4";
const API_KEY: &str = MY_ENV.odds_api_key;

fn print_markets(markets: &Vec<MarketType>) -> String {
    return markets
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");
}

pub fn get_odds_for_sport(
    sport: &str,
    markets: &Vec<MarketType>,
    region: &Region,
) -> reqwest::Result<Vec<Event>> {
    println!(
        "getting odds for sport {0} in region {1}, markets: {2}",
        sport,
        region,
        print_markets(markets)
    );
    let key_usage = get_key_usage();
    match key_usage {
        Some(x) => println!("requests remaining: {0}", x.requests_remaining),
        None => println!(""),
    };

    let odds_endpoint = format!("/sports/{sport}/odds/");
    let full_url = ODDS_HOST_BASE.to_owned() + &odds_endpoint;

    let params = [
        ("apiKey", API_KEY),
        ("regions", &region.to_string()),
        ("markets", &print_markets(markets)),
        ("oddsFormat", "decimal"),
    ];

    let url = reqwest::Url::parse_with_params(&full_url, &params).unwrap();
    let res = match reqwest::blocking::get(url) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    let events = res.json::<Vec<Event>>().unwrap_or(Vec::new());

    let key_usage = get_key_usage();
    match key_usage {
        Some(x) => println!("requests remaining: {0}", x.requests_remaining),
        None => println!(""),
    };

    return Ok(events);
}

pub fn get_key_usage() -> Option<ApiKeyUsage> {
    let response = match get_sports_raw() {
        Ok(x) => x,
        Err(_) => return None,
    };

    let headers = response.headers();
    return get_key_usage_from_headers(&headers);
}

pub fn get_sports_raw() -> Result<Response, Error> {
    let sports_endpoint = "/sports/";
    let full_url = ODDS_HOST_BASE.to_owned() + sports_endpoint;
    let params = [("apiKey", API_KEY)];

    let url = reqwest::Url::parse_with_params(&full_url, &params).unwrap();
    return reqwest::blocking::get(url);
}

pub fn get_sports() -> Result<Vec<Sport>, Error> {
    let sports_endpoint = "/sports/";
    let full_url = ODDS_HOST_BASE.to_owned() + sports_endpoint;
    let params = [("apiKey", API_KEY)];

    let url = reqwest::Url::parse_with_params(&full_url, &params).unwrap();
    let res = match reqwest::blocking::get(url) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    let body = res.json::<Vec<Sport>>().unwrap_or(Vec::new());

    return Ok(body);
}

pub fn get_events(sport_key: &str) -> Result<Vec<Event>, Error> {
    let events_endpoint = format!("/sports/{sport_key}/events");
    let full_url = ODDS_HOST_BASE.to_owned() + &events_endpoint;
    let params = [("apiKey", API_KEY)];

    let url = reqwest::Url::parse_with_params(&full_url, &params).unwrap();
    let res = match reqwest::blocking::get(url) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    let body = res.json::<Vec<Event>>().unwrap_or(Vec::new());

    return Ok(body);
}
