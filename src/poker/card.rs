#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Suit{
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Card {
    pub value: u8,
    pub suit: Suit
}