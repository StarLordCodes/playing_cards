#[derive(Debug, Clone, PartialEq)]
pub enum CardSuite {
    SPADE,
    HEART,
    CLOVER,
    DIAMOND,
}

impl Into<String> for CardSuite {
    fn into(self) -> String {
        match self {
            CardSuite::SPADE => String::from("SPADE"),
            CardSuite::HEART => String::from("HEART"),
            CardSuite::CLOVER => String::from("CLOVER"),
            CardSuite::DIAMOND => String::from("DIAMOND"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CardValue {
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}

impl Into<String> for CardValue {
    fn into(self) -> String {
        match self {
            CardValue::ONE => String::from("ONE"),
            CardValue::TWO => String::from("TWO"),
            CardValue::THREE => String::from("THREE"),
            CardValue::FOUR => String::from("FOUR"),
            CardValue::FIVE => String::from("FIVE"),
            CardValue::SIX => String::from("SIX"),
            CardValue::SEVEN => String::from("SEVEN"),
            CardValue::EIGHT => String::from("EIGHT"),
            CardValue::NINE => String::from("NINE"),
            CardValue::TEN => String::from("TEN"),
            CardValue::JACK => String::from("JACK"),
            CardValue::QUEEN => String::from("QUEEN"),
            CardValue::KING => String::from("KING"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayingCard {
    pub suite: CardSuite,
    pub value: CardValue,
}

impl PlayingCard {
    pub fn new() -> Self {
        Self {
            suite: CardSuite::HEART,
            value: CardValue::KING,
        }
    }
}
