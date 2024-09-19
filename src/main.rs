use clap::Parser;
use discarded::Args;
// DICED PLUS
// A dice roller with no bugs, only features. /j
// TODO:
// - Get the dice type from the user and return an enum
// - Dice:
//   - Roll the dice in the standard way
// - Card:
//   - Determine deck
//   - Draw a random card
// - Color the output
// - Add support for sum and crits

fn main() -> Result<(), ()> {
    let args = Args::parse();
    discarded::handle(args);
    Ok(())
}
