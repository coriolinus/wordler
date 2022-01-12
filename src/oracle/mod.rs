#[cfg(feature = "memory_oracle")]
pub mod memory_oracle;

/// An Oracle knows a secret word and is willing to give feedback regarding the
/// nature of the word.
pub trait Oracle {
    /// Create and initialize the oracle.
    fn new() -> Result<Box<Self>, Error>;

    /// Return the number of characters in the target word.
    fn word_length(&self) -> Result<usize, Error>;

    /// Return whether a word was correct, or feedback if it was not.
    fn guess(&self, guess: &str) -> Result<Result<(), Feedback>, Error>;
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

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("the oracle will answer no more questions")]
    TooManyGuesses,
    #[error("failed to communicate with the oracle")]
    Io(#[source] Box<dyn std::error::Error>),
}
