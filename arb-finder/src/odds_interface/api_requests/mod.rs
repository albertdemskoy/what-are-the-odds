use std::fs;

use chrono::naive::serde::ts_microseconds::serialize;
use reqwest::{blocking::Response, Error};
use util::{get_key_usage_from_headers, ApiKeyUsage};

use super::bookmaker::Region;
use super::market::MarketType;
use super::{event::Event, sport::Sport};
use super::{API_KEY, ODDS_HOST_BASE};

pub mod util;

// todo: these should return the actual type
pub fn get_odds_for_sport_aus(
    sport: &str,
    markets: Vec<MarketType>,
    regions: Vec<Region>,
) -> reqwest::Result<Vec<Event>> {
    let odds_endpoint = format!("/sports/{sport}/odds/");
    let full_url = ODDS_HOST_BASE.to_owned() + &odds_endpoint;

    let params = [
        ("apiKey", API_KEY),
        ("regions", regions.iter().join(",")),
        ("markets", markets.join(",")),
    ];

    let url = reqwest::Url::parse_with_params(&full_url, &params).unwrap();
    let res = match reqwest::blocking::get(url) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    return Ok(res.json::<Vec<Event>>().unwrap_or(Vec::new()));
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
