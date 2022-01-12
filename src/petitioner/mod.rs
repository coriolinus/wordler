#[cfg(feature = "dict_solver")]
pub mod dict_solver;
#[cfg(feature = "human_petitioner")]
pub mod human_petitioner;

/// A petitioner must consider the pronouncements of the Oracle to discover a secret word.
///
/// It must implement a state machine. The proper sequence of calls is:
///
/// - `new`
/// - until a correct guess or oracle guess limit reached:
///     - `prepare_guess`
///     - `feedback`
pub trait Petitioner {
    /// Create a petitioner who will guess words of this number of characters.
    fn new(word_length: usize) -> Result<Box<Self>, Error>;

    /// The petitioner must create a guess satisfying known constraints.
    ///
    /// If called out of sequence, it should return `Error::AwaitingFeedback`.
    fn prepare_guess(&mut self) -> Result<String, Error>;

    /// Send feedback about the previous guess to the petitioner.
    ///
    /// If called out of sequence, it should return `Error::UnexpectedFeedback`.
    fn feedback(&mut self, feedback: crate::oracle::Feedback) -> Result<(), Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to communicate with petitioner")]
    Io(#[source] Box<dyn std::error::Error>),
    #[error("could not determine a word fitting all constraints")]
    Stumped,
    #[error("cannot prepare a new guess while awaiting feedback on a previous guess")]
    AwaitingFeedback,
    #[error("cannot provide new feedback without a new guess")]
    UnexpectedFeedback,
    #[error("feedback provided is inappropriate for the provided guess")]
    InappropriateFeedback,
}
