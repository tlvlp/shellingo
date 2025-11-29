use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone)]
pub struct Question {
    pub question: String,
    pub answers: HashSet<String>,
    pub locations: HashSet<String>,

    correct_count_round: u16,
    error_count_round: u16,
    correct_count_sum: u16,
    error_count_sum: u16,
}

impl Question {
    pub fn new(location: String, question: String, answer: String) -> Question {
        Question {
            question,
            locations: HashSet::from([location]),
            answers: HashSet::from([answer]),

            correct_count_round: 0,
            error_count_round: 0,
            correct_count_sum: 0,
            error_count_sum: 0,
        }
    }
}

impl Question {
    pub fn increment_correct_count(&mut self, amount: u16) {
        self.correct_count_round += amount;
        self.correct_count_sum += amount;
    }

    pub fn increment_error_count(&mut self, amount: u16) {
        self.error_count_round += amount;
        self.error_count_sum += amount;
    }

    pub fn reset_round_stats(&mut self) {
        self.correct_count_round = 0;
        self.error_count_round = 0;
    }

    pub fn get_error_count_for_round(&self) -> u16 {
        self.error_count_round.clone()
    }

    pub fn get_correct_count_for_round(&self) -> u16 {
        self.correct_count_round.clone()
    }

    pub fn get_error_count_sum(&self) -> u16 {
        self.error_count_sum.clone()
    }

    pub fn get_correct_count_sum(&self) -> u16 {
        self.correct_count_sum.clone()
    }
}

impl PartialEq for Question {
    fn eq(&self, other: &Self) -> bool {
        self.question == other.question
    }
}

impl Eq for Question {}

impl Hash for Question {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.question.hash(state);
    }
}
