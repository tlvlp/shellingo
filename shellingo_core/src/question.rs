use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone)]
pub struct Question {
    pub locations: HashSet<String>,
    pub question: String,
    pub solutions: HashSet<String>,

    pub correct_count_round: i32,
    pub error_count_round: i32,
    pub correct_count_sum: i32,
    pub error_count_sum: i32,
}

impl Question {
    pub fn new(location: String, question: String, solution: String) -> Question {
        Question {
            question,
            locations: HashSet::from([location]),
            solutions: HashSet::from([solution]),

            correct_count_round: 0,
            error_count_round: 0,
            correct_count_sum: 0,
            error_count_sum: 0,
        }
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
