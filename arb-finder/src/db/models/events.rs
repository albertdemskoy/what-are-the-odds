use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::{
    db::models::sports::Sport,
    schema::{events, sports::sport_key},
};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Event {
    pub id: i32,
    pub sport_id: i32,
    pub home_team: String,
    pub away_team: String,
    pub commence_time: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub sport_id: i32,
    pub home_team: &'a str,
    pub away_team: &'a str,
    pub commence_time: NaiveDateTime,
}

pub fn create_event(
    conn: &mut PgConnection,
    given_sport_key: &str,
    new_home_team: &str,
    new_away_team: &str,
    new_commence_time: NaiveDateTime,
) -> Option<Event> {
    use crate::schema::sports::dsl::sports;

    let sport = sports
        .filter(sport_key.eq(given_sport_key))
        .select(Sport::as_select())
        .first(conn)
        .expect("Error loading sport");

    let new_event = NewEvent {
        sport_id: sport.id,
        home_team: new_home_team,
        away_team: new_away_team,
        commence_time: new_commence_time,
    };

    diesel::insert_into(events::table)
        .values(&new_event)
        .on_conflict_do_nothing()
        .returning(Event::as_returning())
        .get_result(conn)
        .optional()
        .expect("Error saving new post")
}
