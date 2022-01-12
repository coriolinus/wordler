# Wordler: Wordle from all perspectives

Inspired by [Wordle](https://www.powerlanguage.co.uk/wordle/), which was itself inspired by [Mastermind](https://en.wikipedia.org/wiki/Mastermind_(board_game)).

This provides traits defining both the oracle and the petitioner, and a general game-runner abstracted over those traits.

## Features

- `dict_solver`: A dictionary-based solver implementation.
- `human_petitioner`: IO stuff allowing a human to play interactively at the terminal.
- `memory_oracle`: An in-memory oracle which can initialize itself at random.
- `pretty_feedback`: colorful terminal output mimicing the offical format.
- `wordlist`: Not for public use; establishes a large cache of English words.

## Binaries

### `init-cache`

Min build: `cargo build --release --bin init-cache --features wordlist`.

Just initializes the dictionary cache; doesn't do anything interesting with it.

### `local-wordle`

Min build: `cargo build --release --bin local-wordle --features="human_petitioner memory_oracle"`.

Play unlimited games of wordle against the computer in the terminal. Add `pretty_feedback` to the features list for the best experience.

### `bot-match`

Min build: `cargo build --release --bin bot-match --features="dict_solver memory_oracle"`.

Have the computer play a game of wordle against itself. Add `pretty_feedback` to the features list for the best experience.

Strangely addictive.
