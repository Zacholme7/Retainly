use serde::{Deserialize, Serialize};

/// Flashcards
#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub term: String,
    pub definition: String,
    pub id: Option<i32>,
    pub current_level: usize,
}

impl Card {
    pub fn new(term: String, definition: String) -> Self {
        Self {
            term,
            definition,
            id: None,
            current_level: 1,
        }
    }

    pub fn say_hello() {
        println!("hello");
    }
}

/// Outcome of a card. You got it right or you did not
pub enum Outcome {
    RIGHT,
    WRONG,
}
