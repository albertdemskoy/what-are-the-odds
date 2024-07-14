pub mod api_requests;
pub mod bookiestat;
pub mod bookmaker;
pub mod event;
pub mod market;
pub mod odds;
pub mod sport;

#[cfg(test)]
mod test;

const ODDS_HOST_BASE: &str = "https://api.the-odds-api.com/v4/";
const API_KEY: &str = "74b4c29eb501524fc2d16ca5310de51c";
