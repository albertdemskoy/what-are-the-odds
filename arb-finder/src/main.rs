use common::MarketType;
use db::establish_connection;
use discover_odds::odds_api::db::fetch_and_save_data;
use odds_interface::odds_api::get_key_usage;
use std::io;

mod common;
mod db;
mod discord;
mod discover_odds;
mod local_env;
mod odds_interface;
mod schema;

fn main() {
    let key_usage = get_key_usage();
    match key_usage {
        Some(x) => println!("requests remaining: {0}", x.requests_remaining),
        None => println!(""),
    };

    let enabled_category = "Tennis";
    let connection = &mut establish_connection();
    let enabled_markets = [MarketType::H2h, MarketType::Totals];

    fetch_and_save_data(
        connection,
        &[String::from(enabled_category)].to_vec(),
        &enabled_markets.to_vec(),
    );

    let key_usage = get_key_usage();
    match key_usage {
        Some(x) => println!("requests remaining: {0}", x.requests_remaining),
        None => println!(""),
    };
}
