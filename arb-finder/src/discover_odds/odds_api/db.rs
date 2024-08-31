use chrono::Utc;
use diesel::PgConnection;
use strum::IntoEnumIterator;

use crate::{
    common::MarketType,
    db::models::{
        bookies::get_or_create_book,
        events::get_or_create_event,
        odds::{create_offering, OddsOffering},
        sports::get_or_create_sport,
    },
};

use super::{
    requests::{get_odds_for_sport, get_sports},
    Event, Region,
};

// Odds API Structure
// =====================
// - get all sports in-season (free)
// - get all upcoming events for these sports (also free)
// - for a specific subset of sports,
//     and a specific subset of markets
//     get odds (1 Region at a time) for upcoming events using the /events endpoint

pub fn fetch_and_save_data(
    connection: &mut PgConnection,
    enabled_sports_categories: &Vec<String>,
    enabled_markets: &Vec<MarketType>,
) {
    let sports = get_sports().expect("Failed to get sports");
    for sport in sports {
        if !enabled_sports_categories.contains(&sport.group) {
            continue;
        }

        let db_sport = get_or_create_sport(connection, &sport.key, &sport.group, &sport.title)
            .expect("failed to get or save sport");

        for region in Region::iter() {
            let sport_events = get_odds_for_sport(&sport.key, &enabled_markets.to_vec(), &region)
                .expect("Failed to get events");
            save_odds_to_db(connection, db_sport.id, &sport_events, &region);
        }
    }
}

fn save_odds_to_db(
    connection: &mut PgConnection,
    db_sport_id: i32,
    events: &Vec<Event>,
    region: &Region,
) -> Vec<OddsOffering> {
    let offered_time = Utc::now();

    let new_offerings: Vec<OddsOffering> = Vec::new();
    for event in events {
        let db_event = get_or_create_event(
            connection,
            &db_sport_id,
            &event.home_team,
            &event.away_team,
            event.commence_time,
        )
        .expect("couldn't get or create event");

        if (event.bookmakers.is_none()) {
            return new_offerings;
        }

        for bookie in event.bookmakers.as_ref().unwrap() {
            let db_book = get_or_create_book(
                connection,
                bookie.key.as_str(),
                bookie.title.as_str(),
                &region.to_common_region(),
                bookie.is_exchange(),
            )
            .expect(&format!("couldn't get or create bookie {0}", bookie.key));

            for market in &bookie.markets {
                for outcome in &market.outcomes {
                    create_offering(
                        connection,
                        &db_book.id,
                        &db_event.id,
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
