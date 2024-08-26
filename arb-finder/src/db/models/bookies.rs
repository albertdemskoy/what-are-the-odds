use diesel::prelude::*;
use serde::Serialize;

use crate::common::Region;

#[derive(Queryable, Selectable, Serialize, Identifiable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: i32,
    pub book_key: String,
    pub book_title: String,
    pub region: String,
    pub is_exchange: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::books)]
pub struct NewBook<'a> {
    pub book_key: &'a str,
    pub book_title: &'a str,
    pub region: &'a str,
    pub is_exchange: bool,
}
pub fn book_exists(conn: &mut PgConnection, search_book_key: &str, search_region: &str) -> bool {
    return get_book(conn, search_book_key, search_region).is_some();
}

pub fn get_book(
    conn: &mut PgConnection,
    search_book_key: &str,
    search_region: &str,
) -> Option<Book> {
    use crate::schema::books::dsl::books;
    use crate::schema::books::{book_key, region};

    let maybe_book = books
        .filter(book_key.eq(search_book_key))
        .filter(region.eq(search_region))
        .select(Book::as_select())
        .first(conn);

    return match maybe_book {
        Ok(book) => Some(book),
        Err(_x) => None,
    };
}

pub fn create_book_if_not_exists(
    conn: &mut PgConnection,
    book_key: &str,
    book_title: &str,
    region: &Region,
    is_exchange: bool,
) -> Option<Book> {
    let region_string = region.to_string();
    let new_book = NewBook {
        book_key,
        book_title,
        region: &region_string.as_str(),
        is_exchange,
    };

    diesel::insert_into(crate::schema::books::table)
        .values(&new_book)
        .on_conflict_do_nothing()
        .returning(Book::as_returning())
        .get_result(conn)
        .optional()
        .expect("Error saving new post")
}
