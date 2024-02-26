use crate::db::*;
use chrono::{DateTime, Duration, Utc};
use common::{Card, Outcome};

/// Consuming iterator for the current "day"
struct CardIterator {
    iter: Box<dyn Iterator<Item = Card> + Send>,
}

impl CardIterator {
    /// Construct the iterator
    fn new(iter: Box<dyn Iterator<Item = Card> + Send>) -> Self {
        CardIterator { iter }
    }

    /// Consume the next card for the day
    fn next(&mut self) -> Option<Card> {
        self.iter.next()
    }
}

/// Core structure to maintain learning state
pub struct SpacedRepetition {
    /// The current day that we are on in the review schedule
    pub day: usize,
    /// The review schedule to follow
    review_schedule: ReviewSchedule,
    /// Iterate over the cards for the day
    card_iter: CardIterator,
    /// Signals if we are in the middle of the day
    day_in_progress: bool,
    /// All of the card levels
    levels: Level,
}

impl SpacedRepetition {
    /// Construct a new instance
    pub fn new() -> Self {
        // placeholder for initial card state of the iterator
        let initial_cards: Vec<Card> = Vec::new();

        // read all the cards from the database and insert them into the correct level

        Self {
            day: 1,
            review_schedule: ReviewSchedule::generate_schedule(),
            card_iter: CardIterator::new(Box::new(initial_cards.into_iter())),
            day_in_progress: false,
            levels: Level::default(),
        }
    }

    /// Get the levels that need to be reviewed on this day
    fn levels_to_review(&self) -> Day {
        self.review_schedule.schedule[self.day % self.review_schedule.schedule.len()].clone()
    }

    /// Get the next card in the current review day
    pub fn get_next_card(&mut self) -> Option<Card> {
        // if the day is not currently in progress, we want to start it
        println!("in get next card");
        if self.day_in_progress == false {
            // seup a card iterator and set the day to in progress
            self.init_day_cards();
            self.day_in_progress = true;
        }
        // either return the next card to be reviewed or end the day
        // if we do not have any left in the day
        match self.card_iter.next() {
            Some(card) => Some(card),
            None => {
                self.day_in_progress = false;
                None
            }
        }
    }

    /// At the start of a new day, setup all of the cards that we need to review
    fn init_day_cards(&mut self) {
        // get the levels that we want to review for the day from the review schedule
        println!("in init day cards");
        let levels_for_today = self.levels_to_review();

        // construct the iterator for the days cards
        let cards_for_today = self.get_cards_for_today(&levels_for_today);
        self.card_iter = CardIterator::new(Box::new(cards_for_today.into_iter()));
    }

    /// Create a vector of all the cards that need to be reviewed today
    fn get_cards_for_today(&mut self, levels_to_review: &Day) -> Vec<Card> {
        let mut cards = Vec::new();

        println!("{:?}", self.levels);

        // Iterate over the levels that need to be reviewed today
        for &level_index in &levels_to_review.levels {
            // Use a reference to directly modify the vector in the Level struct
            let level_cards = match level_index {
                1 => &mut self.levels.level_one,
                2 => &mut self.levels.level_two,
                3 => &mut self.levels.level_three,
                4 => &mut self.levels.level_four,
                5 => &mut self.levels.level_five,
                6 => &mut self.levels.level_six,
                7 => &mut self.levels.level_seven,
                _ => panic!("Invalid level index"),
            };

            // Move all cards from this level to the cards vector
            cards.append(level_cards);
            // Since we've moved all cards to the 'cards' vector, the level is now empty
        }
        println!("{:?}", cards);
        cards
    }

    /// Update a card based on our proficiency of it
    pub fn update_card(
        &self,
        outcome: Outcome,
        card: Card,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // get the current level

        // check the outcome
        match outcome {
            YES => {
                // move to next level
                todo!()
            }
            NO => {
                todo!()
                // move back to level one
            }
        }
    }

    /// Inserts a card into level one
    pub fn insert_card_into_level(&mut self, card: Card) {
        self.levels.level_one.push(card);
    }
}

/// Hold all of the cards at different levels
#[derive(Debug)]
pub struct Level {
    pub level_one: Vec<Card>,
    pub level_two: Vec<Card>,
    pub level_three: Vec<Card>,
    pub level_four: Vec<Card>,
    pub level_five: Vec<Card>,
    pub level_six: Vec<Card>,
    pub level_seven: Vec<Card>,
}

// Default constructor for initial instantiation
impl Default for Level {
    fn default() -> Level {
        Level {
            level_one: Vec::new(),
            level_two: Vec::new(),
            level_three: Vec::new(),
            level_four: Vec::new(),
            level_five: Vec::new(),
            level_six: Vec::new(),
            level_seven: Vec::new(),
        }
    }
}

/// The levels that need to be reviwed on a currenty day
#[derive(Clone)]
struct Day {
    levels: Vec<usize>,
}

/// The review schedule
struct ReviewSchedule {
    schedule: Vec<Day>,
}

impl ReviewSchedule {
    /// Generate the review schedule
    pub fn generate_schedule() -> Self {
        Self {
            schedule: { vec![Day { levels: vec![2, 1] }, Day { levels: vec![3, 1] }] },
        }
    }
}
