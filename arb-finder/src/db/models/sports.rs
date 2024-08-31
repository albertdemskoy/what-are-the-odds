use diesel::prelude::*;

use crate::schema::sports;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sports)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sport {
    pub id: i32,
    pub sport_key: String,
    pub category: String,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = sports)]
pub struct NewSport<'a> {
    pub sport_key: &'a str,
    pub category: &'a str,
    pub title: &'a str,
}

fn get_sport(conn: &mut PgConnection, given_sport_key: &str) -> Option<Sport> {
    use crate::schema::sports::dsl::sports;
    use crate::schema::sports::sport_key;

    let maybe_sport = sports
        .filter(sport_key.eq(given_sport_key))
        .select(Sport::as_select())
        .first(conn);

    return match maybe_sport {
        Ok(sport) => Some(sport),
        Err(_x) => None,
    };
}

pub fn get_or_create_sport(
    conn: &mut PgConnection,
    sport_key: &str,
    category: &str,
    title: &str,
) -> Option<Sport> {
    match get_sport(conn, sport_key) {
        Some(x) => return Some(x),
        None => return create_sport(conn, sport_key, category, title),
    }
}

pub fn create_sport(
    conn: &mut PgConnection,
    new_sport_key: &str,
    category: &str,
    title: &str,
) -> Option<Sport> {
    use crate::schema::sports;

    let new_sport = NewSport {
        sport_key: new_sport_key,
        category,
        title,
    };

    diesel::insert_into(sports::table)
        .values(&new_sport)
        .on_conflict_do_nothing()
        .returning(Sport::as_returning())
        .get_result(conn)
        .optional()
        .expect("Error saving new sport")
}
