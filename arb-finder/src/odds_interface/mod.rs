pub mod api_requests;
pub mod odds;
pub mod event;

mod util;

const ODDS_HOST_BASE : &str = "https://api.the-odds-api.com/v4/";
const API_KEY : &str  = "74b4c29eb501524fc2d16ca5310de51c";

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Sport {
    key: String,
    group: String,
    title: String,
    description: String,
    active: bool,
    has_outrights: bool
}

