use rand::prelude::*;
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, PartialEq, EnumIter, Display)]
pub enum CardSuite {
    SPADE,
    HEART,
    CLOVER,
    DIAMOND,
}

#[derive(Debug, Clone, PartialEq, Display, EnumIter)]
pub enum CardValue {
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
    ACE,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayingCard {
    pub suite: CardSuite,
    pub value: CardValue,
}

impl PlayingCard {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let suite = CardSuite::iter().choose(&mut rng).unwrap();
        let value = CardValue::iter().choose(&mut rng).unwrap();
        PlayingCard { suite, value }
    }
}

#[derive(Debug)]
pub struct PlayingCardsDeck(Vec<PlayingCard>);

#[derive(Debug)]
pub struct PlayingCardsHand(Vec<PlayingCard>);
