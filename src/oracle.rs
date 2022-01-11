pub trait Oracle {
    /// Return the number of characters in the target word.
    fn word_length(&self) -> usize;

    /// Return whether a word was correct, or feedback if it was not.
    fn guess(&self, guess: &str) -> Result<(), Feedback>;
}

/// The disposition of a letter indicates how guessers should refine their list of potential words.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disposition {
    /// This letter does not appear in the target word.
    NotInWord,
    /// This letter appears at least once in the target word, elsewhere.
    WrongPosition,
    /// This letter appears in this position in the target word.
    Correct,
    /// The guess was shorter than the target word; this letter is missing.
    Missing,
    /// The guess was longer than the target workd; this letter is extra (and unchecked) otherwise.
    Extra,
}

pub type Feedback = Vec<Disposition>;

// This implementation feels dumb, but for words on the order of 5 chars long, this may actually
// be more efficient than anything more complicated.
pub struct MemoryOracle {
    target: String,
}

impl Oracle for MemoryOracle {
    fn word_length(&self) -> usize {
        self.target.chars().count()
    }

    fn guess(&self, guess: &str) -> Result<(), Feedback> {
        if guess == self.target {
            Ok(())
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

            Err(fb)
        }
    }
}
