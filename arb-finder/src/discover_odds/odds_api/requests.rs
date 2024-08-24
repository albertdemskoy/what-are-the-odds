use std::fs;

use super::util::{get_key_usage_from_headers, ApiKeyUsage};
use super::{Event, MarketType, Region, Sport};
use diesel::PgConnection;
use reqwest::{blocking::Response, Error};

use crate::db::models::odds::OddsOffering;
use crate::local_env::MY_ENV;

const ODDS_HOST_BASE: &str = "https://api.the-odds-api.com/v4";
const API_KEY: &str = MY_ENV.odds_api_key;

pub fn get_odds_for_sport(
    sport: &str,
    markets: &Vec<MarketType>,
    region: &Region,
) -> reqwest::Result<Vec<Event>> {
    let odds_endpoint = format!("/sports/{sport}/odds/");
    let full_url = ODDS_HOST_BASE.to_owned() + &odds_endpoint;

    let params = [
        ("apiKey", API_KEY),
        ("regions", &region.to_string()),
        (
            "markets",
            &markets
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
        ),
        ("oddsFormat", "decimal"),
    ];

    let url = reqwest::Url::parse_with_params(&full_url, &params).unwrap();
    let res = match reqwest::blocking::get(url) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    let events = res.json::<Vec<Event>>().unwrap_or(Vec::new());

    return Ok(events);
}

pub fn get_example_odds_file(filepath: &str) -> Vec<Event> {
    let file_str = fs::read_to_string(filepath).expect("Unable to read file");
    return serde_json::from_str::<Vec<Event>>(&file_str).expect("JSON was not well-formatted");
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
