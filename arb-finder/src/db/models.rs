use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::sports)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sport {
    pub id: i32,
    pub sport_key: String,
    pub category: String,
    pub title: String,
}

use crate::schema::sports;

#[derive(Insertable)]
#[diesel(table_name = sports)]
pub struct NewSport<'a> {
    pub sport_key: &'a str,
    pub category: &'a str,
    pub title: &'a str,
}

pub fn create_sport(
    conn: &mut PgConnection,
    sport_key: &str,
    category: &str,
    title: &str,
) -> Sport {
    let new_post = NewSport {
        sport_key,
        category,
        title,
    };

    diesel::insert_into(sports::table)
        .values(&new_post)
        .returning(Sport::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
