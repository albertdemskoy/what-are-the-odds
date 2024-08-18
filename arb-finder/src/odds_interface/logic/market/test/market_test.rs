use std::collections::HashSet;

use crate::odds_interface::logic::{
    market::{Market, MarketType, Outcome},
    odds::Odds,
};

fn get_market(include_draw: bool) -> Market {
    let no_draw_outcomes = [
        Outcome {
            name: String::from("Sydney Roosters"),
            price: Odds::Decimal(1.2),
            point: None,
        },
        Outcome {
            name: String::from("Wests Tigers"),
            price: Odds::Decimal(5.8),
            point: None,
        },
    ]
    .to_vec();

    let draw_outcome = Outcome {
        name: String::from("Draw"),
        price: Odds::Decimal(10.2),
        point: None,
    };

    let mut all_outcomes = no_draw_outcomes;
    if (include_draw) {
        all_outcomes.push(draw_outcome);
    }

    return Market {
        key: MarketType::H2h,
        outcomes: all_outcomes,
    };
}

#[test]
fn test_get_all_outcomes() {
    let market1 = get_market(true);
    assert_eq!(
        market1.get_all_outcomes(),
        HashSet::from([
            String::from("Sydney Roosters"),
            String::from("Wests Tigers"),
            String::from("Draw")
        ])
    );

    let market2 = get_market(false);
    assert_eq!(
        market2.get_all_outcomes(),
        HashSet::from([
            String::from("Sydney Roosters"),
            String::from("Wests Tigers")
        ])
    );
}

#[test]
fn test_odds_for_outcome() {
    let market = get_market(true);
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
    let market = get_market(true);
    assert_eq!(
        market.total_probability(),
        1.0 / 1.2 + 1.0 / 5.8 + 1.0 / 10.2
    );
}

#[test]
fn test_true_probability_for_outcome() {
    let market = get_market(true);

    assert!(
        (market
            .true_probability_for_outcome("Sydney Roosters")
            .unwrap()
            - 0.799)
            .abs()
            < 0.01
    );

    assert!((market.true_probability_for_outcome("Wests Tigers").unwrap() - 0.137).abs() < 0.01);

    assert!((market.true_probability_for_outcome("Draw").unwrap() - 0.063).abs() < 0.01);
}
