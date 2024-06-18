use std::collections::VecDeque;
use rand::prelude::*;
use strum::EnumIter;
use strum::IntoEnumIterator;
use roman;

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum MajorArcana {
    TheFool,
    TheMagician,
    TheHighPriestess,
    TheEmpress,
    TheEmperor,
    TheHierophant,
    TheLovers,
    TheChariot,
    Strength,
    WheelOfFortune,
    Justice,
    TheHangedMan,
    Death,
    Temperance,
    TheDevil,
    TheTower,
    TheStar,
    TheMoon,
    TheSun,
    Judgement,
    TheWorld,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum MinorArcanaSuits {
    Swords,
    Wands,
    Coins,
    Cups,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum MinorArcanaCardType {
    Number(u8),
    King,
    Queen,
    Knight,
    Page,
}

#[derive(Debug, Clone, Copy)]
pub enum TarotCard {
    GreaterSecret {
        arcana: MajorArcana,
    },
    LesserSecret {
        suit: MinorArcanaSuits,
        value: MinorArcanaCardType,
    },
}

#[derive(Debug, Clone)]
pub struct TarotDeck {
    deck: VecDeque<TarotCard>,
}

impl TarotDeck {
    fn new() -> Self {
        TarotDeck {
            deck: VecDeque::new(),
        }
    }
    fn add(&mut self, card: TarotCard) {
        self.deck.push_back(card);
    }
    pub fn full_deck() -> Self {
        let mut new_deck = TarotDeck::new();
        for card in MajorArcana::iter() {
            new_deck.add(TarotCard::GreaterSecret { arcana: card });
        }
        for suit in MinorArcanaSuits::iter() {
            for number in 1..=10 {
                new_deck.add(TarotCard::LesserSecret {
                    suit,
                    value: MinorArcanaCardType::Number(number),
                });
            }
            for value in MinorArcanaCardType::iter().skip(1) {
                new_deck.add(TarotCard::LesserSecret { suit, value });
            }
        }
        new_deck
    }
    pub fn no_minor_deck() -> Self {
        let mut new_deck = TarotDeck::new();
        for card in MajorArcana::iter() {
            new_deck.add(TarotCard::GreaterSecret { arcana: card });
        }
        new_deck
    }
    pub fn draw(&self, amount: u8) -> Self {
        self.check_draw(amount);
        let mut new_deck = Self::new();
        let mut this_deck = self.clone();
        let mut rng = thread_rng();
        for _ in 0..amount {
            let (index, &out) = this_deck.deck.iter().enumerate().choose(&mut rng).unwrap();
            this_deck.deck.remove(index);
            new_deck.add(out);
        }
        new_deck
    }
    pub fn draw_destructive(&mut self, amount: u8) -> Self {
        self.check_draw(amount);
        let mut new_deck = Self::new();
        let mut rng = thread_rng();
        for _ in 0..amount {
            let (index, &out) = self.deck.iter().enumerate().choose(&mut rng).unwrap();
            self.deck.remove(index);
            new_deck.add(out);
        }
        new_deck
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
impl Iterator for TarotDeck {
    type Item = TarotCard;
    fn next(&mut self) -> Option<Self::Item> {
        self.deck.pop_front()
    }
}
impl std::fmt::Display for TarotCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string())
    }
}
impl TarotCard {
    fn string(&self) -> String {
        match *self {
            Self::GreaterSecret { arcana } => {
                let roman = match arcana {
                    MajorArcana::TheFool => "0".to_string(),
                    _ => roman::to(arcana as i32).unwrap()
                };
                let formatted = format!("{:?}", arcana);
                format!("{} [{}]", camel_case_split(formatted), roman)
            }
            Self::LesserSecret { suit, value } => match value {
                MinorArcanaCardType::Number(val) => format!("{:?} of {:?}", val, suit),
                _ => format!("{:?} of {:?}", value, suit),
            },
        }
    }
}
fn camel_case_split(s: String) -> String {
    s.chars()
        .map(|ch| match ch.is_ascii_uppercase() {
            true => format!("_{ch}").to_string().to_string(),
            _ => format!("{ch}"),
        })
        .collect::<Vec<String>>()
        .join("")
        .replace("_", " ")
        .trim()
        .to_string()
}
