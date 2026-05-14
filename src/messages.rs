use rs_wordle_solver::{GuessResult, LetterResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Guess {
    pub word: String,
    pub marks: [u32; 5],
}

impl Guess {
    pub fn to_solver_guess(&self) -> GuessResult<'_> {
        GuessResult {
            guess: self.word.as_str(),
            results: self
                .marks
                .iter()
                .map(|&c| match c {
                    0 => LetterResult::NotPresent,
                    1 => LetterResult::PresentNotHere,
                    2 => LetterResult::Correct,
                    _ => unreachable!(),
                })
                .collect(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Type {
    Hello { northeastern_username: String },
    Start { id: String },
    Guess { id: String, word: String },
    Retry { id: String, guesses: Vec<Guess> },
    Bye { id: String, flag: String },
    Error { message: String },
}
