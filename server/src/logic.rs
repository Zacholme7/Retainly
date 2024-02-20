use chrono::{DateTime, Duration, Utc};
use common::{Card, Outcome};

/// Consuming iterator for the current "day"
struct CardIterator {
    iter: Box<dyn Iterator<Item = Card>>
}

impl CardIterator {
    /// Construct the iterator
    fn new(iter: Box<dyn Iterator<Item = Card>>) -> Self {
        CardIterator {iter}
    }

    /// Consume the next card for the day
    fn next(&mut self) -> Option<Card> {
        self.iter.next()
    }
}

/// Core structure to maintain learning state
pub struct SpacedRepetition {
    pub day: usize,
    review_schedule: Vec<Review>,
    card_iter: CardIterator,
    day_in_progress: bool,
    levels: Level,
}


impl SpacedRepetition {
    /// Construct a new instance
    pub fn new() -> Self {
        // placeholder for initial card state
        let initial_cards: Vec<Card> = Vec::new();
        Self {
            day: 1,
            review_schedule: Review::generate_schedule(),
            card_iter: CardIterator::new(Box::new(initial_cards.into_iter())),
            day_in_progress: false,
        }
    }

    /// Get the levels that need to be reviewed on this day
    pub fn levels_to_review(&self) -> Review {
        self.review_schedule[self.day & self.review_schedule.len()]
    }

    pub fn get_next_card(&mut self) -> Option<Card> {
        if self.day_in_progress == false {
            self.init_day_cards();
            self.day_in_progress = true;
        }
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
        let levels_for_today = self.levels_to_review();
        let cards_for_today = Vec::new(); // call to get the cards for today
        self.card_iter = CardIterator::new(Box::new(cards_for_today.into_iter()));
    }

    /// Update a card based on our proficiency of it
    pub fn update_card(&self, outcome: Outcome) {
        match outcome {
            YES => todo!(),
            NO => todo!(),
        }
    }
}

/// Hold all of the cards at different levels
pub struct Level {
    pub level_one: Vec<Card>,
    pub level_two: Vec<Card>,
    pub level_three: Vec<Card>,
    pub level_four: Vec<Card>,
    pub level_five: Vec<Card>,
    pub level_six: Vec<Card>,
    pub level_seven: Vec<Card>,
}

/// Represents the levels that need to be reviewed on a day
struct Review {
    level_one: usize,
    level_two: usize,
    level_three: Option<usize>,
}

impl From<(usize, usize, usize)> for Review {
    fn from((level_one, level_two, level_three): (usize, usize, usize)) -> Self {
        Review {
            level_one,
            level_two,
            level_three,
        }
    }
}

impl Review {
    pub fn generate_schedule() -> Vec<Self> {
        vec![Review::from((2, 1, 0))]
    }
}
