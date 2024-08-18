pub mod bookmaker;
pub mod event;
pub mod market;
pub mod odds;
pub mod sport;

#[cfg(test)]
mod test;

const AU_BOOKS: [&str; 13] = [
    "betfair_ex_au",
    "betr_au",
    "betright",
    "bluebet",
    "ladbrokes_au",
    "neds",
    "playup",
    "pointsbetau",
    "sportsbet",
    "tab",
    "tabtouch",
    "topsport",
    "unibet",
];

const AUS_ONLY: bool = false;
