use serde::{Deserialize, Serialize};

/// URL of the webserver
pub const URL: &str = "your aws ip and port";

/// Flashcards
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub term: String,
    pub definition: String,
    pub id: i64,
    pub current_level: i64,
}

impl Card {
    pub fn new(term: String, definition: String) -> Self {
        Self {
            term,
            definition,
            id: -1,
            current_level: 1,
        }
    }
}

/// Outcome of a card. You got it right or you did not
pub enum Outcome {
    RIGHT,
    WRONG,
}

// General information about the state of the application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralInfo {
    pub day: usize,
    pub levels: Vec<usize>,
}
