use ansi_term::Colour::{Blue, Red};
use rand::prelude::*;
use regex::Regex;
use std::{fmt, process};

/// The die model.
#[derive(Debug)]
pub struct Die {
    /// The number of die to roll
    quantity: u16,
    /// The number of sides on the die.
    size: u16,
    /// The number to add/substract to the roll.
    modifier: i16,
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "d{}", self.size);
    }
}

impl Die {
    /// Create a new die using a quantity and size.
    pub fn new(quantity: u16, size: u16, modifier: i16) -> Self {
        if size < 1 {
            eprintln!("Improper Die Size {size}");
            process::exit(1);
        };
        return Self {
            quantity,
            size,
            modifier,
        };
    }

    pub fn size(&self) -> u16 {
        return self.size;
    }

    pub fn quantity(&self) -> u16 {
        return self.quantity;
    }

    pub fn modifier(&self) -> i16 {
        return self.modifier;
    }
}

pub struct DiceArgs {
    pub crit: bool,
    pub count: bool,
    pub sum: bool,
}

fn validate(dice: Vec<String>, match_die: Regex) -> Vec<Die> {
    let mut result: Vec<Die> = vec![];
    for die in dice {
        let capture = match_die.captures(&die).unwrap_or_else(|| {
            eprintln!("[ERR] ~> Die entered improperly: {}", die);
            process::exit(1);
        });
        let size: u16 = capture["size"].parse().unwrap_or_else(|_| {
            eprintln!("[ERR] ~> Die size too large: {}", die);
            eprintln!("[ERR] ~> [Limit is {}]", u16::MAX);
            process::exit(1);
        });
        let quantity: u16 = capture["quantity"].parse().unwrap_or_else(|_| {
            eprintln!("[ERR] ~> Die quantity too large: {}", die);
            eprintln!("[ERR] ~> [Limit is {}]", u16::MAX);
            process::exit(1);
        });
        if let Some(modifier) = capture.get(3) {
            let temp_mod: i16 = modifier.as_str().parse().expect("huh?");
            result.push(Die::new(quantity, size, temp_mod));
        } else {
            result.push(Die::new(quantity, size, 0));
        };
    }
    result
}

pub fn parse(dice: Vec<String>) -> Option<Vec<Die>> {
    if let Ok(match_die) =
        /*
        - This Regex Grabs:
            - The quantity of dice to throw
            - The size (side number) of dice to throw
            - An optional modifier that adds/deducts from the die roll
        */
        Regex::new(r"(?m)(?<quantity>\d+)[d\\/](?<size>\d+)(?<modifier>[\+\-]\d+)?")
    {
        let dice = validate(dice, match_die);
        if dice.len() == 0 {
            eprintln!("[ERR] ~> No die passed in.");
            process::exit(1);
        };
        return Some(dice);
    };
    return None;
}

fn die_format(x: &i16, size: &i16, color: bool) -> String {
    let x_as_str = format!("{}", x);
    if !color {
        return x_as_str;
    };
    match x {
        x if x <= &1 => Red.bold().paint(x_as_str).to_string(),
        x if x >= size => Blue.bold().paint(x_as_str).to_string(),
        _ => x_as_str,
    }
}

pub fn roll_die(die: &Die, arguments: &DiceArgs, rng: &mut ThreadRng) {
    if die.modifier() == 0 {
        println!("{}d{}:", die.quantity(), die.size());
    } else {
        let mod_string = format!(
            "{}{}",
            match die.modifier() {
                i if i < 0 => "-",
                _ => "+",
            },
            die.modifier().abs()
        );
        println!("{}d{} {}:", die.quantity(), die.size(), mod_string);
    }
    let pool: Vec<u16> = vec![0; die.quantity().into()];
    let mut successes: u16 = 0;
    let mut failures: u16 = 0;
    let rolls: Vec<u16> = pool
        .clone()
        .into_iter()
        .map(|_| rng.gen_range(1..=die.size()))
        .collect();
    let sum: u16 = rolls.iter().sum();
    let colored_rolls: Vec<String> = rolls
        .into_iter()
        .inspect(|x| {
            if *x >= die.size() {
                successes += 1
            }
            if *x == 1 {
                failures += 1
            }
        })
        .map(|x| {
            die_format(
                &(die.modifier().wrapping_add(x as i16)),
                &(die.size() as i16),
                arguments.crit,
            )
        })
        .collect();

    if arguments.sum {
        println!("=> ({}): [{}]", colored_rolls.join(", "), sum);
    } else if arguments.count {
        println!(
            "=> ({}): [crit successes: {}, crit failures: {}]",
            colored_rolls.join(", "),
            successes,
            failures
        );
    } else {
        println!("=> ({})", colored_rolls.join(", "));
    }
}
