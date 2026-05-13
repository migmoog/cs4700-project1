use rand::rngs::ThreadRng;
use std::collections::HashSet;

mod guesses;

use guesses::LetterGuess;
use guesses::WORDS_LIST;
use rand::seq::IteratorRandom;

pub struct Wordleizer {
    rng: ThreadRng,
    guesses: HashSet<&'static str>,
    current_guess: [LetterGuess; 5],
}

impl Wordleizer {
    pub fn make_guess(&mut self) -> String {
        self.guesses
            .iter()
            .filter(|word| {
                let char_indices = word.char_indices().collect::<Vec<_>>();
                let mut unguessed_indices = Vec::new();
                let misplaced_chars = self
                    .current_guess
                    .iter()
                    .filter_map(|l| {
                        if let LetterGuess::Other(c) = l {
                            Some(*c)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                for (i, c) in char_indices.iter() {
                    if let LetterGuess::Correct(correct) = self.current_guess.as_slice()[*i] {
                        if correct != *c {
                            return false;
                        }
                    } else {
                        unguessed_indices.push(*i);
                    }
                }

                if !unguessed_indices.is_empty() && !misplaced_chars.is_empty() {
                    char_indices
                        .iter()
                        .all(|(i, c)| unguessed_indices.contains(i) && misplaced_chars.contains(c))
                } else {
                    true
                }
            })
            .choose(&mut self.rng)
            .expect("should have a remaining guess")
            .to_string()
    }
}

impl Default for Wordleizer {
    fn default() -> Self {
        let guesses = WORDS_LIST.lines().collect();

        Self {
            guesses,
            rng: rand::rng(),
            current_guess: Default::default(),
        }
    }
}
