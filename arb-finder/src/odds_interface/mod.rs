use crate::local_env::MY_ENV;

pub mod api_requests;
pub mod bookiestat;
pub mod bookmaker;
pub mod event;
pub mod market;
pub mod odds;
pub mod sport;

#[cfg(test)]
mod test;

const ODDS_HOST_BASE: &str = "https://api.the-odds-api.com/v4";
const API_KEY: &str = MY_ENV.odds_api_key;
