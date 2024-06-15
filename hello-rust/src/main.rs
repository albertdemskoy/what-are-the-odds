use reqwest::Error;
use reqwest::blocking::Response;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct Sport {
    key: String,
    group: String,
    title: String,
    description: String,
    active: bool,
    has_outrights: bool
}

fn get_sports() -> Result<Response, Error> {

    let odds_host_base = "https://api.the-odds-api.com/v4/";
    let sports_endpoint = "/sports/";
    let full_url =  odds_host_base.to_owned() + sports_endpoint;
    let params = [
        ("apiKey", "74b4c29eb501524fc2d16ca5310de51c"),
    ];

    let url = reqwest::Url::parse_with_params(&full_url, &params).unwrap();
    let res = reqwest::blocking::get(url);

    return res;
}

fn main() {

    let response = match get_sports() {
        Ok(res) => res,
        Err(_e) => return,
    };

    let headers = response.headers();
    println!("headers:\n=======================\n{headers:?}"); 

    let body = response.json::<Vec<Sport>>().unwrap();
    println!("body:\n===========================\n{body:?}");    
}
