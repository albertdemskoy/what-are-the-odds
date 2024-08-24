use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use crate::odds_interface::logic_old::market::MarketType;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::odds_offering)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct OddsOffering {
    pub id: i32,
    pub book_id: i32,
    pub event_id: i32,
    pub outcome: String,
    pub offered_at: DateTime<Utc>,
    pub market_type: String,
    pub offered_line: Option<BigDecimal>,
    pub offered_odds: BigDecimal,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::odds_offering)]
pub struct NewOddsOffering<'a> {
    pub book_id: i32,
    pub event_id: i32,
    pub outcome: &'a str,
    pub offered_at: &'a DateTime<Utc>,
    pub market_type: &'a str,
    pub offered_line: Option<&'a BigDecimal>,
    pub offered_odds: &'a BigDecimal,
}

pub fn create_offering(
    conn: &mut PgConnection,
    assoc_book_key: &str,
    event_id: i32,
    outcome: &str,
    offered_at: &DateTime<Utc>,
    assoc_market_type: &MarketType,
    offered_line: Option<&BigDecimal>,
    offered_odds: &BigDecimal,
) -> Option<OddsOffering> {
    use crate::schema::books::dsl::books;
    use crate::{db::models::bookies::Book, schema::books::book_key};

    let book = books
        .filter(book_key.eq(assoc_book_key))
        .select(Book::as_select())
        .first(conn)
        .expect("Error loading sport");

    let market_type_string = assoc_market_type.to_string();

    let new_offering = NewOddsOffering {
        book_id: book.id,
        outcome,
        event_id,
        offered_at,
        market_type: &market_type_string.as_str(),
        offered_line,
        offered_odds,
    };

    diesel::insert_into(crate::schema::odds_offering::table)
        .values(&new_offering)
        .on_conflict_do_nothing()
        .returning(OddsOffering::as_returning())
        .get_result(conn)
        .optional()
        .expect("Error saving new post")
}
