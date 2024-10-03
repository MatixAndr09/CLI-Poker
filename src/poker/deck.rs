use super::card::{Card, Suit};
use rand::seq::SliceRandom;

pub struct Deck{
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for &suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for value in 2..=14{
                cards.push(Card { value, suit: suit.clone()});
            }
        }

        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.as_mut_slice().shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card>{
        self.cards.pop();
    }
}