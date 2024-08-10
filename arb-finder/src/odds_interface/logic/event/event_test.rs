use chrono::DateTime;

use crate::odds_interface::logic::event::Event;

fn get_event() -> Event {
    return Event {
        id: String::from("1"),
        sport_key: String::from("aussierules_afl"),
        commence_time: DateTime::parse_from_rfc3339("2024-08-04T03:10:23Z")
            .unwrap_or_default()
            .to_utc(),
        home_team: String::from("Greater Western Sydney Giants"),
        away_team: String::from("Hawthorn Hawks"),
        bookmakers: [].to_vec(),
    };
}
