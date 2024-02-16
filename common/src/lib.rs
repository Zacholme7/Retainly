use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub term: String,
    pub definition: String,
}

impl Card {
    pub fn new(term: String, definition: String) -> Self {
        Self { term, definition }
    }

    pub fn say_hello() {
        println!("hello");
    }
}
