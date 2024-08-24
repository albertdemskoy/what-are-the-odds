use serde::Deserialize;
use strum_macros::EnumIter;

#[derive(Deserialize, Debug, Clone, PartialEq, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum MarketType {
    H2h,
    H2hLay,
    Spreads,
    Totals,
    Outrights,
    OutrightsLay,
}
