use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Guess {
    word: String,
    marks: [u32; 5],
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
