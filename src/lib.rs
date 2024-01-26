use rand::Rng;

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
        let suite = rand::thread_rng().gen_range(0..4);
        let value = rand::thread_rng().gen_range(0..13);

        Self {
            suite: match suite {
                0 => CardSuite::SPADE,
                1 => CardSuite::HEART,
                2 => CardSuite::CLOVER,
                _ => CardSuite::DIAMOND,
            },
            value: match value {
                0 => CardValue::ONE,
                1 => CardValue::TWO,
                2 => CardValue::THREE,
                3 => CardValue::FOUR,
                4 => CardValue::FIVE,
                5 => CardValue::SIX,
                6 => CardValue::SEVEN,
                7 => CardValue::EIGHT,
                8 => CardValue::NINE,
                9 => CardValue::TEN,
                10 => CardValue::JACK,
                11 => CardValue::QUEEN,
                _ => CardValue::KING,
            },
        }
    }
}

#[derive(Debug)]
pub struct PlayingCardsDeck(Vec<PlayingCard>);

#[derive(Debug)]
pub struct PlayingCardsHand(Vec<PlayingCard>);
