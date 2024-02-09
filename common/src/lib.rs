use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    term: String,
    definition: String,
}

impl Card {
    pub fn new(term: &str, definition: &str) -> Self {
        println!("this is being called");
        Self { term: term.to_string(), definition: definition.to_string() }
    }

    pub fn say_hello() {
        println!("hello");
    }
}

