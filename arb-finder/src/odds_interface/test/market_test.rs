use std::collections::HashSet;

use chrono::DateTime;

use crate::odds_interface::{
    market::{Market, MarketType, Outcome},
    odds::Odds,
};

fn get_market() -> Market {
    return Market {
        key: MarketType::H2h,
        outcomes: [
            Outcome {
                name: String::from("Sydney Roosters"),
                price: Odds::Decimal(1.2),
            },
            Outcome {
                name: String::from("Wests Tigers"),
                price: Odds::Decimal(5.8),
            },
            Outcome {
                name: String::from("Draw"),
                price: Odds::Decimal(10.2),
            },
        ]
        .to_vec(),
    };
}

#[test]
fn test_get_all_outcomes() {
    let market = get_market();
    assert_eq!(
        market.get_all_outcomes(),
        HashSet::from([
            String::from("Sydney Roosters"),
            String::from("Wests Tigers"),
            String::from("Draw")
        ])
    )
}

#[test]
fn test_odds_for_outcome() {
    let market = get_market();
    assert_eq!(
        market.odds_for_outcome("Sydney Roosters"),
        Some(Odds::Decimal(1.2))
    );
    assert_eq!(
        market.odds_for_outcome("Wests Tigers"),
        Some(Odds::Decimal(5.8))
    );
    assert_eq!(market.odds_for_outcome("Draw"), Some(Odds::Decimal(10.2)));
}

#[test]
fn test_total_probability() {
    let market = get_market();
    assert_eq!(
        market.total_probability(),
        1.0 / 1.2 + 1.0 / 5.8 + 1.0 / 10.2
    );
}
