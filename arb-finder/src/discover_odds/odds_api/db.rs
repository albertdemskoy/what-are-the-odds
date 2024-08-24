use chrono::Utc;
use diesel::PgConnection;

use crate::db::models::{
    bookies::create_book_if_not_exists,
    events::get_event,
    odds::{create_offering, OddsOffering},
    sports::get_sport,
};

use super::{Event, Region};

pub fn save_odds_to_db(
    conn: &mut PgConnection,
    sport_key: &str,
    events: &Vec<Event>,
    region: &Region,
) -> Vec<OddsOffering> {
    let maybe_db_sport = get_sport(conn, sport_key);
    let db_sport = match maybe_db_sport {
        Some(x) => x,
        None => return Vec::new(),
    };

    let offered_time = Utc::now();

    let new_offerings: Vec<OddsOffering> = Vec::new();
    for event in events {
        let maybe_db_event = get_event(
            conn,
            event.home_team.as_str(),
            event.away_team.as_str(),
            event.commence_time,
        );

        let db_event = match maybe_db_event {
            Some(x) => x,
            None => continue,
        };

        if (event.bookmakers.is_none()) {
            return new_offerings;
        }

        for bookie in event.bookmakers.as_ref().unwrap() {
            create_book_if_not_exists(
                conn,
                bookie.key.as_str(),
                bookie.title.as_str(),
                &region.to_common_region(),
                bookie.is_exchange(),
            );

            for market in &bookie.markets {
                for outcome in &market.outcomes {
                    create_offering(
                        conn,
                        bookie.key.as_str(),
                        db_event.id,
                        outcome.name.as_str(),
                        &offered_time,
                        &market.key,
                        outcome.point.as_ref(),
                        &outcome.price,
                    );
                }
            }
        }
    }

    return new_offerings;
}
