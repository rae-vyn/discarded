use card::Deck;
use clap::{Parser, Subcommand};
use die::{parse, roll_die, DiceArgs};
use rand::prelude::*;
pub mod card;
pub mod die;
pub mod tarot;

/// Diced: a dice roller/card drawer with no bugs, only features!
#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Draw a card from a deck.
    Draw {
        #[command(subcommand)]
        drawcommand: DrawCommand,
    },
    /// Roll a die.
    Roll {
        /// The dice to roll.
        dice: Vec<String>,

        /// Color critical successes and fails
        #[arg(long)]
        crit: bool,

        /// Count the number of successes and fails.
        #[arg(long)]
        count: bool,

        /// Add up all of the rolls.
        #[arg(long)]
        sum: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum DrawCommand {
    /// A traditional deck of playing cards.
    Traditional {
        /// Draw [AMOUNT] number of cards from the deck.
        amount: Option<u8>,
        /// Use a deck without jokers.
        #[arg(long)]
        no_jokers: bool,
        /// Draw nondestructively
        #[arg(long)]
        nondestructive: bool,
    },
    /// A 78-card tarot deck, optionally including the minor arcana.
    Tarot {
        /// Draw [AMOUNT] number of cards from the deck.
        amount: Option<u8>,
        /// Include the minor arcana.
        #[arg(long)]
        include_minor: bool,
        /// Draw nondestructively
        #[arg(long)]
        nondestructive: bool,
    },
}

pub fn handle(args: Args) {
    match args.command {
        Commands::Draw { drawcommand } => match drawcommand {
            DrawCommand::Traditional {
                amount,
                no_jokers,
                nondestructive,
            } => {
                let used_amount = match amount {
                    Some(val) => val,
                    _ => 1,
                };
                let mut used_deck = match no_jokers {
                    true => Deck::deck_no_jokers(),
                    false => Deck::deck(),
                };
                if !nondestructive {
                    for card in used_deck.draw_destructive(used_amount) {
                        println!("{card}");
                    }
                } else {
                    for card in used_deck.draw(used_amount) {
                        println!("{card}");
                    }
                }
            }
            DrawCommand::Tarot {
                amount,
                include_minor,
                nondestructive,
            } => {
                let used_amount = match amount {
                    Some(val) => val,
                    _ => 1,
                };

                let mut deck = match include_minor {
                    true => tarot::TarotDeck::full_deck(),
                    false => tarot::TarotDeck::no_minor_deck(),
                };
                if !nondestructive {
                    for card in deck.draw_destructive(used_amount) {
                        println!("{card}");
                    }
                } else {
                    for card in deck.draw(used_amount) {
                        println!("{card}");
                    }
                }
            }
        },
        Commands::Roll {
            dice,
            crit,
            count,
            sum,
        } => {
            let dice = parse(dice);
            let args = DiceArgs { crit, count, sum };
            match dice {
                Some(dice) => {
                    for die in dice {
                        roll_die(&die, &args, &mut thread_rng())
                    }
                }
                None => std::process::exit(0),
            };
        }
    };
}
