#[cfg(test)]
use playing_cards::{CardSuite, CardValue, PlayingCard};

#[test]
fn test_suite() {
    // test string
    assert_eq!(CardSuite::HEART.to_string(), String::from("HEART"));

    // test debug
    assert_eq!(format!("{:?}", CardSuite::CLOVER), "CLOVER");
}

#[test]
fn test_value() {
    // test string
    assert_eq!(CardValue::KING.to_string(), String::from("KING"));

    // test debug
    assert_eq!(format!("{:?}", CardValue::QUEEN), "QUEEN");
}

#[test]
fn test_playing_card_struct() {
    // test playing_card
    let playing_card_new = PlayingCard::new();
    assert_eq!("1", "2");
    println!("{:?}", playing_card_new);
}
