use rand::prelude::*;
use std::collections::VecDeque;
use std::fmt;
use strum::EnumIter;
use strum::IntoEnumIterator;
// The Card Model
#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardFaceType {
    Number(u8),
    King,
    Queen,
    Jack,
    Ace,
    BigJoker,
    LittleJoker,
}

#[derive(Debug, EnumIter, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardColor {
    Red,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub face_value: CardFaceType,
    pub suit: CardSuit,
    color: CardColor,
}

impl Card {
    pub fn new(value: CardFaceType, suit: CardSuit) -> Self {
        let color = match suit {
            CardSuit::Hearts | CardSuit::Diamonds => CardColor::Red,
            CardSuit::Spades | CardSuit::Clubs => CardColor::Black,
            CardSuit::None => match value {
                CardFaceType::BigJoker => CardColor::Black,
                _ => CardColor::Red,
            },
        };
        Self {
            face_value: value,
            suit,
            color: color,
        }
    }
    pub fn color(&self) -> CardColor {
        self.color
    }
    pub fn num_val(&self) -> Option<u8> {
        match self.face_value {
            CardFaceType::Number(x) => Some(x),
            _ => None,
        }
    }
    pub fn string(&self) -> String {
        let face_as_string = match self.face_value {
            CardFaceType::Number(x) => format!("{}", x),
            CardFaceType::BigJoker => "Big Joker".to_string(),
            CardFaceType::LittleJoker => "Little Joker".to_string(),
            other => format!("{:?}", other),
        };
        match self.suit {
            CardSuit::None => format!("{}", face_as_string),
            _ => format!("{} of {:?}", face_as_string, self.suit),
        }
    }
}
impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.string())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Deck {
    deck: VecDeque<Card>,
}

impl Deck {
    pub fn new(deck: Vec<Card>) -> Self {
        Self { deck: deck.into() }
    }
    pub fn new_empty() -> Self {
        Self {
            deck: vec![].into(),
        }
    }
    pub fn deck() -> Deck {
        let mut deck = Deck::new(vec![]);
        for suit in CardSuit::iter().take(4) {
            // All the numbers
            for i in 2..=10 {
                deck.add(Card::new(CardFaceType::Number(i), suit));
            }
            // The big people
            for face in CardFaceType::iter().skip(1).take(4) {
                deck.add(Card::new(face, suit));
            }
        }
        // Add the two jokers
        deck.add(Card::new(CardFaceType::BigJoker, CardSuit::None));
        deck.add(Card::new(CardFaceType::LittleJoker, CardSuit::None));
        deck
    }
    pub fn deck_no_jokers() -> Deck {
        let mut deck = Deck::new(vec![]);
        for suit in CardSuit::iter().take(4) {
            // All the numbers
            for i in 2..=10 {
                deck.add(Card::new(CardFaceType::Number(i), suit));
            }
            // The big people
            for face in CardFaceType::iter().skip(1).take(4) {
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
        let mut new_deck = Deck::new(vec![]);
        let _ = self
            .deck
            .iter()
            .inspect(|card| {
                if condition(**card) {
                    new_deck.add(**card)
                }
            })
            .collect::<Vec<&Card>>();
        new_deck
    }
    pub fn size(&self) -> u8 {
        self.deck.len() as u8
    }
    pub fn draw(&self, amount: u8) -> Deck {
        self.check_draw(amount);
        let mut new_deck = Deck::new_empty();
        let mut this_deck = self.clone();
        let mut rng = thread_rng();
        for _ in 0..amount {
            let (index, &out) = this_deck.deck.iter().enumerate().choose(&mut rng).unwrap();
            this_deck.deck.remove(index);
            new_deck.add(out);
        }
        new_deck
    }
    pub fn draw_destructive(&mut self, amount: u8) -> Deck {
        self.check_draw(amount);
        let mut new_deck = Deck::new_empty();
        let mut rng = thread_rng();
        for _ in 0..amount {
            let (index, &out) = self.deck.iter().enumerate().choose(&mut rng).unwrap();
            self.deck.remove(index);
            new_deck.add(out);
        }
        new_deck
    }
    pub fn raw(&self) -> Vec<Card> {
        self.deck.to_owned().into()
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
