use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::{DateTime, Utc};
use diesel::{
    prelude::*,
    result::Error,
    sql_query,
    sql_types::{Integer, Text},
};
use serde::Serialize;

use super::{bookies::Book, events::Event};
use crate::common::{bookie_odds::BookieWithOdds, MarketType};

#[derive(
    Queryable,
    QueryableByName,
    Selectable,
    Associations,
    Identifiable,
    Debug,
    PartialEq,
    Serialize,
    Clone,
)]
#[diesel(belongs_to(Book))]
#[diesel(belongs_to(Event))]
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

impl OddsOffering {
    pub fn is_for_market(&self, market_type: &MarketType) -> bool {
        return MarketType::from_str(self.market_type.as_str()).is_some_and(|x| x == *market_type);
    }

    pub fn implied_probability(&self) -> f64 {
        return 1.0 / self.float_odds();
    }

    pub fn float_odds(&self) -> f64 {
        return self.offered_odds.to_f64().unwrap();
    }
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

impl OddsOffering {
    pub fn latest(
        conn: &mut PgConnection,
        event: &Event,
        market: &MarketType,
    ) -> Result<Vec<BookieWithOdds>, Error> {
        use crate::schema::books;

        let all_bookies = books::table.select(Book::as_select()).load(conn)?;

        // let odds = OddsOffering::belonging_to(&all_bookies)
        //     .filter(market_type.eq(market.to_string()))
        //     .filter(event_id.eq(event.id))
        //     .order(offered_at.desc())
        //     .select(OddsOffering::as_select())
        //     .load(conn)?;

        let query = r#"
            SELECT *
            FROM (
                SELECT *,
                       ROW_NUMBER() OVER (PARTITION BY (book_id, outcome, market_type) ORDER BY offered_at DESC) AS rn
                FROM odds_offering
                WHERE event_id = ?
            ) subquery
            WHERE rn = 1
        "#;

        let odds = sql_query(query)
            .bind::<Integer, _>(event.id)
            .bind::<Text, _>(market.to_string())
            .get_results(conn)
            .expect("An error has occured");

        let odds_per_bookie = odds
            .grouped_by(&all_bookies)
            .into_iter()
            .zip(all_bookies)
            .map(|(odds_offerings, bookie)| BookieWithOdds {
                bookie,
                odds_offerings,
            })
            .collect::<Vec<BookieWithOdds>>();

        return Ok(odds_per_bookie);
    }
}

pub fn create_offering(
    conn: &mut PgConnection,
    book_id: &i32,
    event_id: &i32,
    outcome: &str,
    offered_at: &DateTime<Utc>,
    assoc_market_type: &MarketType,
    offered_line: Option<&BigDecimal>,
    offered_odds: &BigDecimal,
) -> Option<OddsOffering> {
    let market_type_string = assoc_market_type.to_string();

    let new_offering = NewOddsOffering {
        book_id: book_id.clone(),
        outcome,
        event_id: event_id.clone(),
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
