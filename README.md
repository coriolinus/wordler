# Wordler: Wordle from all perspectives

Inspired by [Wordle](https://www.powerlanguage.co.uk/wordle/), which was itself inspired by [Mastermind](https://en.wikipedia.org/wiki/Mastermind_(board_game)).

This provides traits defining both the oracle and the petitioner, and a general game-runner abstracted over those traits.

## Features

- `memory_oracle`: An in-memory oracle which can initialize itself at random.
- `wordlist`: Not necessary for public use; establishes a large cache of English words.
