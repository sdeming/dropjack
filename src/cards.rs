use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    pub fn symbol(&self) -> &str {
        match self {
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Suit::Hearts | Suit::Diamonds => Color::Red,
            Suit::Spades | Suit::Clubs => Color::Black,
        }
    }

    pub fn all() -> Vec<Suit> {
        vec![Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Value {
    pub fn symbol(&self) -> &str {
        match self {
            Value::Ace => "A",
            Value::Two => "2",
            Value::Three => "3",
            Value::Four => "4",
            Value::Five => "5",
            Value::Six => "6",
            Value::Seven => "7",
            Value::Eight => "8",
            Value::Nine => "9",
            Value::Ten => "10",
            Value::Jack => "J",
            Value::Queen => "Q",
            Value::King => "K",
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            Value::Ace => 1, // Ace can be 1 or 11, handled in game logic
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten | Value::Jack | Value::Queen | Value::King => 10,
        }
    }

    pub fn all() -> Vec<Value> {
        vec![
            Value::Ace,
            Value::Two,
            Value::Three,
            Value::Four,
            Value::Five,
            Value::Six,
            Value::Seven,
            Value::Eight,
            Value::Nine,
            Value::Ten,
            Value::Jack,
            Value::Queen,
            Value::King,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Self {
        Card { suit, value }
    }

    // For Ace, we need to check if it should be 1 or 11
    pub fn blackjack_values(&self) -> Vec<u8> {
        if self.value == Value::Ace {
            vec![1, 11]
        } else {
            vec![self.value.value()]
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value.symbol(), self.suit.symbol())
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let cards = Suit::all()
            .into_iter()
            .flat_map(|suit| {
                Value::all().into_iter().map(move |value| Card::new(suit, value))
            })
            .collect();
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn reset(&mut self) {
        *self = Deck::new();
        self.shuffle();
    }
}

// Raylib color enum for card rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}
