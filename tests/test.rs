#[cfg(test)]
use playing_cards::{CardSuite, CardValue, PlayingCard};

#[test]
fn test_suite() {
    // test string
    let suite_str: String = CardSuite::HEART.into();
    assert_eq!(suite_str, String::from("HEART"));

    // test debug
    assert_eq!(format!("{:?}", CardSuite::CLOVER), "CLOVER");
}

#[test]
fn test_value() {
    // test string
    let suite_str: String = CardValue::KING.into();
    assert_eq!(suite_str, String::from("KING"));

    // test debug
    assert_eq!(format!("{:?}", CardValue::QUEEN), "QUEEN");
}

#[test]
fn test_playing_card_struct() {
    // test playing_card
    let playing_card_new = PlayingCard::new();
    let playing_card_expected = PlayingCard {
        suite: CardSuite::HEART,
        value: CardValue::KING,
    };
    assert_eq!(playing_card_expected, playing_card_new);
}
