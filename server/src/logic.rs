use chrono::{DateTime, Duration, Utc};
use common::{Card, Outcome};

pub struct SpacedRepetition {
    pub day: usize,
    review_schedule: Vec<Review>,
}

struct Review {
    level_one: usize,
    level_two: usize,
    level_three: usize,
}

impl SpacedRepetition {}

impl SpacedRepetition {
    pub fn new() -> Self {
        Self {
            day: 1,
            review_schedule: Review::generate_schedule(),
        }
    }

    /// Get the levels that need to be reviewed on this day
    pub fn levels_to_review(&self) -> Review {
        self.review_schedule[self.day - 1]
    }

    pub fn get_next_card(&self) -> Option<Card> {
        None
    }

    pub fn update_card(&self, outcome: Outcome) {
        match outcome {
            YES => todo!(),
            NO => todo!(),
        }
    }
}

pub struct Level {
    pub level_one: Vec<Card>,
    pub level_two: Vec<Card>,
    pub level_three: Vec<Card>,
    pub level_four: Vec<Card>,
    pub level_five: Vec<Card>,
    pub level_six: Vec<Card>,
    pub level_seven: Vec<Card>,
}

impl Level {
    /// Retrieve the next card that should be reviewed
    pub fn get_next_card(&self) -> Option<Card> {
        todo!()
    }
    pub fn present_card(&mut self, card: Card, current_level: usize, success: bool) {
        // give the card to the client
        // get if it answered it correctly
        // if it did, move it to the next level and

        /*
        let next_review = match current_level {
            1 => Utc::now() + Duration::days(1),
            2 => Utc::now() + Duration::days(2),
            3 => Utc::now() + Duration::days(4),
            4 => Utc::now() + Duration::days(8),
            5 => Utc::now() + Duration::days(16),
            6 => Utc::now() + Duration::days(32),
            _ => Utc::now() + Duration::days(64), // For level 7 and beyond, you might keep the
                                                  // same interval, or adjust as neededk
        };
        */

        // send the card here
    }
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
