use std::fmt;

use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub mod bookie_odds;
pub mod market_info;
pub mod util;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, EnumIter, Hash, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MarketType {
    H2h,
    H2hLay,
    Spreads,
    Totals,
    Outrights,
    OutrightsLay,
}

#[derive(Deserialize, Debug, Clone, PartialEq, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum Region {
    Us,
    Uk,
    Au,
    Eu,
}

impl fmt::Display for MarketType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            MarketType::H2h => write!(f, "h2h"),
            MarketType::H2hLay => write!(f, "h2h_lay"),
            MarketType::Spreads => write!(f, "spreads"),
            MarketType::Totals => write!(f, "totals"),
            MarketType::Outrights => write!(f, "outrights"),
            MarketType::OutrightsLay => write!(f, "outrights_lay"),
        }
    }
}

impl MarketType {
    pub fn from_str(arg: &str) -> Option<MarketType> {
        for market_type in MarketType::iter() {
            if (market_type.to_string() == arg.to_string()) {
                return Some(market_type);
            }
        }
        return None;
    }
}

impl Region {
    pub fn from_str(arg: &str) -> Option<Region> {
        for region in Region::iter() {
            if (region.to_string() == arg.to_string()) {
                return Some(region);
            }
        }
        return None;
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Region::Us => write!(f, "us"),
            Region::Uk => write!(f, "uk"),
            Region::Au => write!(f, "au"),
            Region::Eu => write!(f, "eu"),
        }
    }
}
