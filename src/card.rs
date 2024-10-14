use rand::prelude::*;
use std::collections::VecDeque;
use std::fmt;
use strum::EnumIter;
use strum::IntoEnumIterator;
// The Card Model
#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FaceType {
    Number(u8),
    King,
    Queen,
    Jack,
    Ace,
    BigJoker,
    LittleJoker,
}

#[derive(Debug, EnumIter, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub face_value: FaceType,
    pub suit: Suit,
    color: Color,
}

impl Card {
    #[must_use]
    pub const fn new(value: FaceType, suit: Suit) -> Self {
        let color = match suit {
            Suit::Hearts | Suit::Diamonds => Color::Red,
            Suit::Spades | Suit::Clubs => Color::Black,
            Suit::None => match value {
                FaceType::BigJoker => Color::Black,
                _ => Color::Red,
            },
        };
        Self {
            face_value: value,
            suit,
            color,
        }
    }
    #[must_use] pub const fn color(&self) -> Color {
        self.color
    }
    #[must_use] pub const fn num_val(&self) -> Option<u8> {
        match self.face_value {
            FaceType::Number(x) => Some(x),
            _ => None,
        }
    }
    #[must_use] pub fn string(&self) -> String {
        let face_as_string = match self.face_value {
            FaceType::Number(x) => format!("{x}"),
            FaceType::BigJoker => "Big Joker".to_string(),
            FaceType::LittleJoker => "Little Joker".to_string(),
            other => format!("{other:?}"),
        };
        match self.suit {
            Suit::None => face_as_string,
            _ => format!("{} of {:?}", face_as_string, self.suit),
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deck {
    deck: VecDeque<Card>,
}

impl Deck {
    #[must_use] pub fn new(deck: Vec<Card>) -> Self {
        Self { deck: deck.into() }
    }
    #[must_use] pub fn new_empty() -> Self {
        Self {
            deck: vec![].into(),
        }
    }
    #[must_use] pub fn deck() -> Self {
        let mut deck = Self::new(vec![]);
        for suit in Suit::iter().take(4) {
            // All the numbers
            for i in 2..=10 {
                deck.add(Card::new(FaceType::Number(i), suit));
            }
            // The big people
            for face in FaceType::iter().skip(1).take(4) {
                deck.add(Card::new(face, suit));
            }
        }
        // Add the two jokers
        deck.add(Card::new(FaceType::BigJoker, Suit::None));
        deck.add(Card::new(FaceType::LittleJoker, Suit::None));
        deck
    }
    #[must_use] pub fn deck_no_jokers() -> Self {
        let mut deck = Self::new(vec![]);
        for suit in Suit::iter().take(4) {
            // All the numbers
            for i in 2..=10 {
                deck.add(Card::new(FaceType::Number(i), suit));
            }
            // The big people
            for face in FaceType::iter().skip(1).take(4) {
                deck.add(Card::new(face, suit));
            }
        }
        deck
    }
    pub fn add(&mut self, card: Card) {
        self.deck.push_back(card);
    }
    pub fn rem_end(&mut self) {
        Into::<Vec<Card>>::into(self.deck.clone()).pop();
    }
    pub fn filter_cards(&self, condition: fn(Card) -> bool) -> Self {
        let mut new_deck = Self::new(vec![]);
        let _ = self
            .deck
            .iter()
            .inspect(|card| {
                if condition(**card) {
                    new_deck.add(**card);
                }
            })
            .collect::<Vec<&Card>>();
        new_deck
    }
    #[must_use] pub fn size(&self) -> u8 {
        self.deck.len() as u8
    }
    #[must_use] pub fn draw(&self, amount: u8) -> Self {
        let mut new_deck = Self::new_empty();
        let mut rng = thread_rng();
        for _ in 0..amount {
            let &out = self.deck.iter().choose(&mut rng).unwrap();
            new_deck.add(out);
        }
        new_deck
    }
    pub fn draw_destructive(&mut self, amount: u8) -> Self {
        self.check_draw(amount);
        let mut new_deck = Self::new_empty();
        let mut rng = thread_rng();
        for _ in 0..amount {
            let (index, &out) = self.deck.iter().enumerate().choose(&mut rng).unwrap();
            self.deck.remove(index);
            new_deck.add(out);
        }
        new_deck
    }
    #[must_use] pub fn raw(&self) -> Vec<Card> {
        self.deck.clone().into()
    }
    pub fn check_draw(&self, amount: u8) {
        if amount > self.deck.len() as u8 {
            eprintln!(
                "[ERR] -> {} cards requested, deck only has {} cards.",
                amount,
                self.deck.len()
            );
            std::process::exit(0);
        }
    }
}
impl Iterator for Deck {
    type Item = Card;
    fn next(&mut self) -> Option<Self::Item> {
        self.deck.pop_front()
    }
}
