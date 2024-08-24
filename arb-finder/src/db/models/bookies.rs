use diesel::prelude::*;

#[derive(Queryable, Selectable)]
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

pub fn create_book(
    conn: &mut PgConnection,
    book_key: &str,
    book_title: &str,
    region: &str,
    is_exchange: bool,
) -> Option<Book> {
    let new_book = NewBook {
        book_key,
        book_title,
        region,
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
