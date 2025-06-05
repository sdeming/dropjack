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

    pub fn color(&self) -> CardColor {
        match self {
            Suit::Hearts | Suit::Diamonds => CardColor::Red,
            Suit::Spades | Suit::Clubs => CardColor::Black,
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
                Value::all()
                    .into_iter()
                    .map(move |value| Card::new(suit, value))
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
pub enum CardColor {
    Red,
    Black,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suit_symbol() {
        assert_eq!(Suit::Spades.symbol(), "♠");
        assert_eq!(Suit::Hearts.symbol(), "♥");
        assert_eq!(Suit::Diamonds.symbol(), "♦");
        assert_eq!(Suit::Clubs.symbol(), "♣");
    }

    #[test]
    fn test_suit_color() {
        assert_eq!(Suit::Hearts.color(), CardColor::Red);
        assert_eq!(Suit::Diamonds.color(), CardColor::Red);
        assert_eq!(Suit::Spades.color(), CardColor::Black);
        assert_eq!(Suit::Clubs.color(), CardColor::Black);
    }

    #[test]
    fn test_suit_all() {
        let all_suits = Suit::all();
        assert_eq!(all_suits.len(), 4);
        assert!(all_suits.contains(&Suit::Spades));
        assert!(all_suits.contains(&Suit::Hearts));
        assert!(all_suits.contains(&Suit::Diamonds));
        assert!(all_suits.contains(&Suit::Clubs));
    }

    #[test]
    fn test_value_symbol() {
        assert_eq!(Value::Ace.symbol(), "A");
        assert_eq!(Value::Two.symbol(), "2");
        assert_eq!(Value::Ten.symbol(), "10");
        assert_eq!(Value::Jack.symbol(), "J");
        assert_eq!(Value::Queen.symbol(), "Q");
        assert_eq!(Value::King.symbol(), "K");
    }

    #[test]
    fn test_value_value() {
        assert_eq!(Value::Ace.value(), 1);
        assert_eq!(Value::Two.value(), 2);
        assert_eq!(Value::Five.value(), 5);
        assert_eq!(Value::Ten.value(), 10);
        assert_eq!(Value::Jack.value(), 10);
        assert_eq!(Value::Queen.value(), 10);
        assert_eq!(Value::King.value(), 10);
    }

    #[test]
    fn test_value_all() {
        let all_values = Value::all();
        assert_eq!(all_values.len(), 13);
        assert!(all_values.contains(&Value::Ace));
        assert!(all_values.contains(&Value::King));
    }

    #[test]
    fn test_card_new() {
        let card = Card::new(Suit::Hearts, Value::Ace);
        assert_eq!(card.suit, Suit::Hearts);
        assert_eq!(card.value, Value::Ace);
    }

    #[test]
    fn test_card_blackjack_values() {
        let ace = Card::new(Suit::Hearts, Value::Ace);
        let ace_values = ace.blackjack_values();
        assert_eq!(ace_values, vec![1, 11]);

        let king = Card::new(Suit::Spades, Value::King);
        let king_values = king.blackjack_values();
        assert_eq!(king_values, vec![10]);

        let five = Card::new(Suit::Clubs, Value::Five);
        let five_values = five.blackjack_values();
        assert_eq!(five_values, vec![5]);
    }

    #[test]
    fn test_card_display() {
        let card = Card::new(Suit::Hearts, Value::Ace);
        assert_eq!(format!("{}", card), "A♥");

        let card2 = Card::new(Suit::Spades, Value::King);
        assert_eq!(format!("{}", card2), "K♠");
    }

    #[test]
    fn test_deck_new() {
        let deck = Deck::new();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_deck_contains_all_cards() {
        let deck = Deck::new();

        // Check that we have exactly 4 of each value
        for value in Value::all() {
            let count = deck.cards.iter().filter(|card| card.value == value).count();
            assert_eq!(count, 4, "Should have 4 cards of value {:?}", value);
        }

        // Check that we have exactly 13 of each suit
        for suit in Suit::all() {
            let count = deck.cards.iter().filter(|card| card.suit == suit).count();
            assert_eq!(count, 13, "Should have 13 cards of suit {:?}", suit);
        }
    }

    #[test]
    fn test_deck_draw() {
        let mut deck = Deck::new();
        let initial_count = deck.cards.len();

        let card = deck.draw();
        assert!(card.is_some());
        assert_eq!(deck.cards.len(), initial_count - 1);

        // Draw all cards
        while deck.draw().is_some() {}
        assert_eq!(deck.cards.len(), 0);

        // Drawing from empty deck should return None
        assert!(deck.draw().is_none());
    }

    #[test]
    fn test_deck_reset() {
        let mut deck = Deck::new();

        // Draw some cards
        deck.draw();
        deck.draw();
        deck.draw();
        assert_eq!(deck.cards.len(), 49);

        // Reset the deck
        deck.reset();
        assert_eq!(deck.cards.len(), 52);
    }

    #[test]
    fn test_deck_shuffle() {
        let mut deck1 = Deck::new();
        let mut deck2 = Deck::new();

        // Store original order
        let original_order = deck1.cards.clone();

        // Shuffle one deck
        deck1.shuffle();

        // It's extremely unlikely (but theoretically possible) that shuffle produces
        // the same order, so we'll test this multiple times
        let mut shuffled_at_least_once = false;
        for _ in 0..10 {
            deck2.shuffle();
            if deck2.cards != original_order {
                shuffled_at_least_once = true;
                break;
            }
        }
        assert!(
            shuffled_at_least_once,
            "Deck should be shuffled after multiple attempts"
        );
    }

    mod test_fixtures {
        use super::*;

        pub fn create_test_deck_with_specific_cards() -> Deck {
            let cards = vec![
                Card::new(Suit::Hearts, Value::Ace),
                Card::new(Suit::Spades, Value::King),
                Card::new(Suit::Diamonds, Value::Queen),
                Card::new(Suit::Clubs, Value::Ten),
            ];
            Deck { cards }
        }

        pub fn create_blackjack_hand() -> Vec<Card> {
            vec![
                Card::new(Suit::Hearts, Value::Ace),
                Card::new(Suit::Spades, Value::King),
            ]
        }

        pub fn create_bust_hand() -> Vec<Card> {
            vec![
                Card::new(Suit::Hearts, Value::King),
                Card::new(Suit::Spades, Value::Queen),
                Card::new(Suit::Diamonds, Value::Five),
            ]
        }
    }

    #[test]
    fn test_fixture_specific_deck() {
        let deck = test_fixtures::create_test_deck_with_specific_cards();
        assert_eq!(deck.cards.len(), 4);
        assert_eq!(deck.cards[0], Card::new(Suit::Hearts, Value::Ace));
        assert_eq!(deck.cards[3], Card::new(Suit::Clubs, Value::Ten));
    }

    #[test]
    fn test_fixture_blackjack_hand() {
        let hand = test_fixtures::create_blackjack_hand();
        assert_eq!(hand.len(), 2);

        // Calculate hand value (Ace + King = 21)
        let mut total = 0;
        let mut aces = 0;

        for card in &hand {
            if card.value == Value::Ace {
                aces += 1;
                total += 11; // Start with 11 for Ace
            } else {
                total += card.value.value() as i32;
            }
        }

        // Adjust for Aces if total > 21
        while total > 21 && aces > 0 {
            total -= 10; // Convert Ace from 11 to 1
            aces -= 1;
        }

        assert_eq!(total, 21);
    }

    #[test]
    fn test_fixture_bust_hand() {
        let hand = test_fixtures::create_bust_hand();
        let total: u8 = hand.iter().map(|card| card.value.value()).sum();
        assert!(total > 21);
    }
}
