# Diced

A simple CLI dice rolling tool.


Usage: 

```
diced_plus roll [OPTIONS] [DICE]...

Arguments:
  [DICE]...  The dice to roll

Options:
      --crit   Color critical successes and fails
      --count  Count the number of successes and fails
      --sum    Add up all of the rolls

diced_plus draw <COMMAND>

Commands:
  traditional  A traditional deck of playing cards
  tarot        A 78-card tarot deck, optionally including the minor arcana
  help         Print this message or the help of the given subcommand(s)

diced_plus draw traditional [OPTIONS] [AMOUNT]

Arguments:
  [AMOUNT]  Draw [AMOUNT] number of cards from the deck

Options:
      --no-jokers  Use a deck without jokers

diced_plus draw tarot [OPTIONS] [AMOUNT]

Arguments:
  [AMOUNT]  Draw [AMOUNT] number of cards from the deck

Options:
      --include-minor  Include the minor arcana
```
