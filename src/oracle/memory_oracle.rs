use std::cell::Cell;

use crate::oracle::{Disposition, Error, Feedback, Oracle};
use rand::seq::IteratorRandom;

// This implementation feels dumb, but for words on the order of 5 chars long, this may actually
// be more efficient than anything more complicated.
pub struct MemoryOracle {
    target: String,
    guesses: Cell<usize>,
    pub max_guesses: Option<usize>,
}

impl Oracle for MemoryOracle {
    fn new() -> Result<Box<Self>, Error> {
        // by default we pick a 5-character word
        Self::create_random(5)
    }

    fn word_length(&self) -> Result<usize, Error> {
        Ok(self.target.chars().count())
    }

    fn guess(&self, guess: &str) -> Result<Result<(), Feedback>, Error> {
        let mut guesses = self.guesses.get();
        guesses += 1;
        self.guesses.set(guesses);

        if self
            .max_guesses
            .map(|max_guesses| guesses > max_guesses)
            .unwrap_or_default()
        {
            Err(Error::TooManyGuesses)
        } else if guess == self.target {
            Ok(Ok(()))
        } else {
            let mut fb = Feedback::with_capacity(guess.len());
            for (have, want) in guess.chars().zip(self.target.chars()) {
                if have == want {
                    fb.push(Disposition::Correct);
                } else if self.target.contains(have) {
                    fb.push(Disposition::WrongPosition);
                } else {
                    fb.push(Disposition::NotInWord);
                }
            }

            let target_chars = self.target.chars().count();
            let guess_chars = guess.chars().count();
            fb.extend(
                std::iter::repeat(Disposition::Missing)
                    .take(target_chars.saturating_sub(guess_chars)),
            );
            fb.extend(
                std::iter::repeat(Disposition::Extra)
                    .take(guess_chars.saturating_sub(target_chars)),
            );

            debug_assert!(fb.len() >= target_chars);
            debug_assert!(fb.len() <= guess_chars);
            debug_assert!(!fb.iter().all(|&disp| disp == Disposition::Correct));

            Ok(Err(fb))
        }
    }
}

impl MemoryOracle {
    fn create_random(characters: usize) -> Result<Box<Self>, Error> {
        let mut rng = rand::thread_rng();

        let word = crate::wordlist::load()
            .map_err(|err| Error::Io(Box::new(err)))?
            .filter(|word| word.chars().count() == characters)
            .choose(&mut rng)
            .expect("word list was empty");

        Ok(Box::new(Self {
            target: word,
            max_guesses: None,
            guesses: Cell::new(0),
        }))
    }
}
