use chrono::{DateTime, Utc};
use diesel::{prelude::*, result::Error};
use serde::Serialize;

use crate::{
    db::models::sports::Sport,
    schema::{
        events,
        sports::{self, category},
    },
};

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Associations, Serialize, Clone)]
#[diesel(table_name = crate::schema::events)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Sport))]
pub struct Event {
    pub id: i32,
    pub sport_id: i32,
    pub home_team: String,
    pub away_team: String,
    pub commence_time: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct NewEvent<'a> {
    pub sport_id: i32,
    pub home_team: &'a str,
    pub away_team: &'a str,
    pub commence_time: DateTime<Utc>,
}

impl Event {
    pub fn upcoming_for_sport(
        conn: &mut PgConnection,
        sport_category: &str,
        after_time: DateTime<Utc>,
        until_time: DateTime<Utc>,
    ) -> Result<Vec<Event>, Error> {
        use crate::schema::events::commence_time;
        use crate::schema::events::dsl::events;

        return events
            .left_join(sports::table)
            .filter(commence_time.lt(until_time))
            .filter(commence_time.gt(after_time))
            .filter(category.eq(sport_category))
            .select(Event::as_select())
            .load(conn);
    }
}

pub fn get_event(
    conn: &mut PgConnection,
    search_sport_id: &i32,
    search_home_team: &str,
    search_away_team: &str,
    search_commence_time: DateTime<Utc>,
) -> Option<Event> {
    use crate::schema::events::dsl::events;
    use crate::schema::events::{away_team, commence_time, home_team, sport_id};

    let maybe_event = events
        .filter(sport_id.eq(search_sport_id))
        .filter(home_team.eq(search_home_team))
        .filter(away_team.eq(search_away_team))
        .filter(commence_time.eq(search_commence_time))
        .select(Event::as_select())
        .first(conn);

    return match maybe_event {
        Ok(event) => Some(event),
        Err(_x) => None,
    };
}

pub fn get_or_create_event(
    conn: &mut PgConnection,
    sport_id: &i32,
    new_home_team: &str,
    new_away_team: &str,
    new_commence_time: DateTime<Utc>,
) -> Option<Event> {
    match get_event(
        conn,
        sport_id,
        new_home_team,
        new_away_team,
        new_commence_time,
    ) {
        Some(x) => return Some(x),
        None => {
            return create_event(
                conn,
                sport_id,
                new_home_team,
                new_away_team,
                new_commence_time,
            )
        }
    };
}

pub fn create_event(
    conn: &mut PgConnection,
    sport_id: &i32,
    new_home_team: &str,
    new_away_team: &str,
    new_commence_time: DateTime<Utc>,
) -> Option<Event> {
    let new_event = NewEvent {
        sport_id: sport_id.clone(),
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
