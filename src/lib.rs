pub mod oracle;
pub mod petitioner;
#[cfg(feature = "wordlist")]
pub mod wordlist;

pub use oracle::Oracle;
pub use petitioner::Petitioner;

#[cfg(feature = "pretty_feedback")]
fn print_feedback(guess: &str, feedback: &crate::oracle::FeedbackRef) {
    use crate::oracle::Disposition::*;
    use itertools::Itertools;
    use std::io::Write;
    use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

    let not_in_word = {
        let mut cs = ColorSpec::new();
        cs.set_fg(Some(Color::White));
        cs.set_bg(Some(Color::Black));
        cs
    };
    let wrong_position = {
        let mut cs = ColorSpec::new();
        cs.set_fg(Some(Color::White));
        cs.set_bg(Some(Color::Yellow));
        cs.set_intense(true);
        cs
    };
    let correct = {
        let mut cs = ColorSpec::new();
        cs.set_fg(Some(Color::White));
        cs.set_bg(Some(Color::Green));
        cs.set_intense(true);
        cs.set_bold(true);
        cs
    };
    let error = {
        let mut cs = ColorSpec::new();
        cs.set_fg(Some(Color::White));
        cs.set_bg(Some(Color::Red));
        cs.set_dimmed(true);
        cs
    };

    let mut out = StandardStream::stdout(ColorChoice::Always);
    for eob in guess.chars().zip_longest(feedback.iter().copied()) {
        match eob {
            itertools::EitherOrBoth::Both(ch, disposition) => {
                match disposition {
                    NotInWord => out.set_color(&not_in_word),
                    WrongPosition => out.set_color(&wrong_position),
                    Correct => out.set_color(&correct),
                    Extra => out.set_color(&error),
                    Missing => unreachable!("missing char was present"),
                }
                .expect("setting color works properly");
                write!(out, "{}", ch).expect("sending char to terminal works properly");
            }
            itertools::EitherOrBoth::Left(_) => {
                unreachable!("feedback always at least as long as guess")
            }
            itertools::EitherOrBoth::Right(disposition) => {
                if disposition == Missing {
                    out.set_color(&error).expect("setting color works properly");
                    write!(out, "X").expect("sending char to terminal works properly");
                } else {
                    unreachable!("extra disposition chars were not 'missing'")
                }
            }
        }
    }

    out.reset().expect("clearing colors should just work");
    writeln!(out).expect("sending char to terminal works properly");
}

#[cfg(not(feature = "pretty_feedback"))]
fn print_feedback(guess: &str, feedback: &crate::oracle::FeedbackRef) {
    println!("guess: {}", guess);
    println!("feedback: {:?}", feedback);
}

/// Run a game of wordle according to the oracle and petitioner.
pub fn wordle<Oracle, Petitioner>(show_feedback: bool) -> Result<String, Box<dyn std::error::Error>>
where
    Oracle: oracle::Oracle,
    Petitioner: petitioner::Petitioner,
{
    let oracle = Oracle::new()?;
    let word_length = oracle.word_length()?;
    let mut petitioner = Petitioner::new(word_length);

    loop {
        let guess = petitioner.prepare_guess()?;
        match oracle.guess(&guess)? {
            Ok(_) => {
                return Ok(guess);
            }
            Err(feedback) => {
                if show_feedback {
                    print_feedback(&guess, &feedback);
                }
                petitioner.feedback(feedback)?;
            }
        }
    }
}
