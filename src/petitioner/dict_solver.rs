//! This module uses a dictionary-based approach to solve wordle problems.

use itertools::Itertools;
use rand::seq::SliceRandom;

pub struct DictSolver {
    /// the words which we can choose from
    words: Vec<String>,
    /// the previous guess
    last_guess: Option<String>,
    /// the set of characters known to be in the right place
    known_chars: Vec<Option<char>>,
    /// chars known to be in the word but not in this position
    known_wrong_chars: Vec<Vec<char>>,
    /// the set of characters known to be in the word in unknown position
    unknown_position: Vec<char>,
    /// the set of characters known not to be in the word
    not_in_word: Vec<char>,
}

impl crate::Petitioner for DictSolver {
    fn new(word_length: usize) -> Result<Box<Self>, super::Error> {
        let words = crate::wordlist::load()
            .map_err(|err| super::Error::Io(Box::new(err)))?
            .filter(|word| word.chars().count() == word_length)
            .collect();
        Ok(Box::new(Self {
            words,
            last_guess: None,
            known_chars: vec![None; word_length],
            known_wrong_chars: vec![Vec::new(); word_length],
            unknown_position: Vec::with_capacity(5),
            not_in_word: Vec::with_capacity(26 - 5),
        }))
    }

    fn prepare_guess(&mut self) -> Result<String, super::Error> {
        if self.last_guess.is_some() {
            return Err(super::Error::AwaitingFeedback);
        }

        let mut rng = rand::thread_rng();
        self.last_guess = self.words.choose(&mut rng).cloned();
        self.last_guess.clone().ok_or(super::Error::Stumped)
    }

    fn feedback(&mut self, feedback: crate::oracle::Feedback) -> Result<(), super::Error> {
        use crate::oracle::Disposition::*;

        let last_guess = match self.last_guess.take() {
            Some(last_guess) => last_guess,
            None => return Err(super::Error::UnexpectedFeedback),
        };

        for (idx, eob) in last_guess.chars().zip_longest(feedback).enumerate() {
            match eob {
                itertools::EitherOrBoth::Both(ch, disp) => match disp {
                    NotInWord => {
                        self.not_in_word.push(ch);
                    }
                    WrongPosition => {
                        self.unknown_position.push(ch);
                        self.known_wrong_chars[idx].push(ch);
                    }
                    Correct => {
                        self.known_chars[idx] = Some(ch);
                        self.unknown_position.retain(|unk| unk != &ch);
                    }
                    Missing | Extra => {
                        unreachable!("this petitioner always submits words of the right length")
                    }
                },
                itertools::EitherOrBoth::Left(_) => {
                    return Err(super::Error::InappropriateFeedback)
                }
                itertools::EitherOrBoth::Right(_) => {
                    unreachable!("this petitioner always submits words of the correct length")
                }
            }
        }

        self.words.retain(|word| {
            // eliminate words with characters known to be wrong
            if word.chars().any(|ch| self.not_in_word.contains(&ch)) {
                return false;
            }
            // choose only words with characters matching known-good characters
            if word
                .chars()
                .zip_eq(self.known_chars.iter())
                .any(|(have, want)| want.map(|want| want != have).unwrap_or_default())
            {
                return false;
            }
            // choose only words with characters which are not in known-wrong places
            if word
                .chars()
                .zip_eq(self.known_wrong_chars.iter())
                .any(|(have, wrong_list)| wrong_list.iter().contains(&have))
            {
                return false;
            }
            // choose only words which contain all necessary characters
            for need_ch in self.unknown_position.iter() {
                if !word.chars().contains(need_ch) {
                    return false;
                }
            }

            // other words must be ok
            true
        });

        Ok(())
    }
}
