use std::{borrow::BorrowMut, fs};

use crate::odds_interface::logic::{
    event::Event,
    market::{MarketType, Outcome},
    odds::Odds,
};

/// ### Input data:
/// - **Matchup**: *"Brisbane Lions"* vs *"St Kilda Saints"*
/// - **Bookies**: *sportsbet*, *tab*, *unibet*, *pointsbetau*
///
/// Each has the same odds, with
/// - **h2h**: 1.62 lions 2.3 saints
/// - **totals**: 166.5, 1.9 each way
/// - **spreads**: -7.5 to Lions, 1.9 each way
///
fn get_afl_event() -> Event {
    let raw_file_string =
        fs::read_to_string("./src/odds_interface/logic/event/event_test/testdata_afl.json")
            .expect("Unable to read file");
    return serde_json::from_str::<Event>(&raw_file_string).expect("JSON was not well-formatted");
}

impl Event {
    fn update_line(&mut self, bookie_key: &str, new_line: f64) {
        let bookie = self
            .bookmakers
            .iter_mut()
            .find(|x| x.key == bookie_key)
            .unwrap();

        let market = bookie
            .markets
            .iter_mut()
            .find(|x| x.key == MarketType::Totals)
            .unwrap();

        market.outcomes = market
            .outcomes
            .iter()
            .map(|x| Outcome {
                name: x.name.clone(),
                price: x.price,
                point: Some(new_line),
            })
            .collect();
    }

    fn update_odds(
        &mut self,
        bookie_key: &str,
        market_type: &MarketType,
        outcome_key: &str,
        odds_decimal: f64,
    ) {
        let bookie = self
            .bookmakers
            .iter_mut()
            .find(|x| x.key == bookie_key)
            .unwrap();

        let market = bookie
            .markets
            .iter_mut()
            .find(|x| x.key == *market_type)
            .unwrap();

        let mut outcome = market
            .outcomes
            .iter_mut()
            .find(|x| x.name == outcome_key)
            .unwrap();

        let outcome_ref = &mut outcome;

        outcome_ref.price = Odds::Decimal(odds_decimal);
    }
}

#[test]
fn test_get_h2h_opportunities() {
    let mut event = get_afl_event();
    let opportunities = event.identify_opportunities();

    // NO OPPORTUNITIES
    assert_eq!(opportunities.len(), 0);

    // change all odds except sportsbet to suddenly favour the saints a bit more
    event.update_odds("tab", &MarketType::H2h, "St Kilda Saints", 1.95);
    event.update_odds("tab", &MarketType::H2h, "Brisbane Lions", 1.7);

    event.update_odds("unibet", &MarketType::H2h, "St Kilda Saints", 1.95);
    event.update_odds("unibet", &MarketType::H2h, "Brisbane Lions", 1.7);

    event.update_odds("pointsbetau", &MarketType::H2h, "St Kilda Saints", 1.95);
    event.update_odds("pointsbetau", &MarketType::H2h, "Brisbane Lions", 1.7);

    let opportunities = event.identify_opportunities();

    assert_eq!(opportunities.len(), 1);
    let first_opp = opportunities.first().unwrap();
    assert_eq!(first_opp.bookie_name, "SportsBet");
    assert_eq!(first_opp.outcome_key, "St Kilda Saints");
}

#[test]
fn test_get_totals_opportunities_high_score() {
    let mut event = get_afl_event();
    let opportunities = event.identify_opportunities();

    // NO OPPORTUNITIES
    assert_eq!(opportunities.len(), 0);

    // change all odds except sportsbet to suddenly favour the saints a bit more
    event.update_line("tab", 170.5);
    event.update_line("unibet", 170.5);
    event.update_line("pointsbetau", 170.5);

    let opportunities = event.identify_opportunities();

    assert_eq!(opportunities.len(), 1);
    let first_opp = opportunities.first().unwrap();
    assert_eq!(first_opp.bookie_name, "SportsBet");
    assert_eq!(first_opp.outcome_key, "Over");
}
