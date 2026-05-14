mod guesses;

use guesses::WORDS_LIST;
use rs_wordle_solver::{Guesser, RandomGuesser, WordBank};
use crate::messages::Guess;

pub struct Wordleizer {
    guesser: RandomGuesser,
}

impl Wordleizer {
    pub fn make_guess(&mut self) -> String {
        self.guesser.select_next_guess()
            .expect("Should have a remaining guess")
            .to_string()
    }

    pub fn adjust(&mut self, guess: &Guess) {
        self.guesser.update(&guess.to_solver_guess()).unwrap()
    }
}

impl Default for Wordleizer {
    fn default() -> Self {
        let wordbank = WordBank::from_iterator(WORDS_LIST.lines()).unwrap();
        Self {
            guesser: RandomGuesser::new(wordbank),
        }
    }
}
