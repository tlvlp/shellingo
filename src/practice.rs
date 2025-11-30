use std::cell::RefCell;
use std::rc::Rc;
use std::sync::LazyLock;
use regex::Regex;
use crate::question::Question;

pub const CLUE_REVEAL_PENALTY: u16 = 5;
pub const ANSWER_REVEAL_PENALTY: u16 = 10;
static REGEX_MULTIPLE_SPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());
static REGEX_SYMBOLS_TO_REMOVE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[?,!.:;]+").unwrap());

pub fn reveal_clue(question: Rc<RefCell<Question>>) -> String {
    question.borrow().answers.iter()
        .next()
        .map(|answer|
            answer.chars()
                .enumerate()
                // Mask every second character
                .map(|(index, c)|  if c == ' ' || index % 2 == 0 { c } else { '■' })
                .collect::<String>()
        )
        .unwrap_or(format!("Cannot generate clue for the answer(s): '{:?}'", question))
}

pub fn reveal_answer(question: Rc<RefCell<Question>>) -> String {
    question.borrow_mut().answers.iter()
        .next()
        .unwrap_or_else(|| panic!("Cannot reveal answer(s): '{:?}'", question))
        .clone()
}

pub fn get_hardest_questions_in_round(questions: &[Rc<RefCell<Question>>], limit: usize) -> Vec<Rc<RefCell<Question>>> {
    // Reverse sort (hardest first)
    let mut refs = questions.to_vec();
    refs.sort_by_key(|question|
        std::cmp::Reverse(question.borrow().get_error_count_for_round()));
    // Keep only the first X items
    refs.into_iter()
        .take(limit)
        .collect()
}

pub fn is_attempt_successful(attempt: &str, question: Rc<RefCell<Question>>) -> bool {
    let cleaned_attempt = clean_string(attempt);
    question.borrow_mut().answers.iter()
        .map(|answer| clean_string(answer) == cleaned_attempt)
        .reduce(|a, b| a || b)
        .unwrap_or(false)
}

fn clean_string(response: &str) -> String {
    let trimmed_lowercase = response
        .trim()
        .to_lowercase();
    let single_space = REGEX_MULTIPLE_SPACES.replace_all(&trimmed_lowercase, " ");
    REGEX_SYMBOLS_TO_REMOVE.replace_all(single_space.as_ref(), "").to_string()

}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn reveal_clue_for_for_single_answer() {
        // Given
        let question = Rc::new(RefCell::new(Question::new("location_1".to_string(), "question_1".to_string(), "answer_1".to_string())));
        let expected = "a■s■e■_■".to_string();

        // When
        let actual = reveal_clue(question.clone());

        // Then
        assert_eq!(expected, actual);
        assert!(question.borrow_mut().answers.contains("answer_1"), "The original answer remains unchanged");
    }

    #[test]
    fn reveal_clue_with_multiple_answers() {
        // Given
        let question = Rc::new(RefCell::new(Question::new("location_1".to_string(), "question_1".to_string(), "placeholder".to_string())));
        question.borrow_mut().answers  = HashSet::from(["answer_one".to_string(), "answer_two".to_string()]);
        let expected = "a■s■e■_■n■".to_string();
        let expected_variant = "a■s■e■_■w■".to_string();

        // When
        let actual = reveal_clue(question);

        // Then
        assert!(expected == actual || expected_variant == actual); // HashSet ordering can be random.
    }


    #[test]
    fn reveal_answer_for_single_answer() {
        // Given
        let question =  Rc::new(RefCell::new(Question::new("location_1".to_string(), "question_1".to_string(), "answer_1".to_string())));
        let expected = "answer_1".to_string();

        // When
        let actual = reveal_answer(question);

        // Then
        assert_eq!(expected, actual);
    }

    #[test]
    fn reveal_answer_for_with_multiple_answers() {
        // Given
        let question =  Rc::new(RefCell::new(Question::new("location_1".to_string(), "question_1".to_string(), "placeholder".to_string())));
        question.borrow_mut().answers = HashSet::from(["answer_one".to_string(), "answer_two".to_string()]);

        let expected = "answer_one".to_string();
        let expected_variant = "answer_two".to_string();

        // When
        let actual = reveal_answer(question);

        // Then
        assert!(expected == actual || expected_variant == actual); // HashSet ordering can be random.

    }

    #[test]
    fn test_get_hardest_questions_in_round() {
        // Given
        let q1 = Rc::new(RefCell::new(Question::new(String::new(), String::from("q1"), String::new())));
        let q2 = Rc::new(RefCell::new(Question::new(String::new(), String::from("q2"), String::new())));
        let q3 = Rc::new(RefCell::new(Question::new(String::new(), String::from("q3"), String::new())));
        let q4 = Rc::new(RefCell::new(Question::new(String::new(), String::from("q3"), String::new())));
        // Expected order: q4, q2, q3
        q2.borrow_mut().increment_error_count(5);
        q3.borrow_mut().increment_error_count(1);
        q4.borrow_mut().increment_error_count(10);

        let limit = 3;

        let questions = vec![q1.clone(), q2.clone(), q3.clone(), q4.clone()];
        let expected = vec![questions[3].clone(), questions[1].clone(), questions[2].clone()]; // Will drop q1, due to the limit.

        // When
        let actual = get_hardest_questions_in_round(&questions, limit);

        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn is_attempt_successful_matches_answer() {
        // Given
        let question = Rc::new(RefCell::new(Question::new(String::new(), String::from("q1"), String::new())));
        question.borrow_mut().answers = HashSet::from(["answer one".to_string(), "answer two".to_string()]);
        let attempt = "answer one";

        // When
        let actual = is_attempt_successful(attempt, question);

        //Then
        assert_eq!(actual, true);
    }


    #[test]
    fn is_attempt_successful_no_answer_to_match() {
        // Given
        let question = Rc::new(RefCell::new(Question::new(String::new(), String::from("q1"), String::new())));
        question.borrow_mut().answers = HashSet::from(["answer one".to_string(), "answer two".to_string()]);
        let attempt = "something else";

        // When
        let actual = is_attempt_successful(attempt, question);

        //Then
        assert_eq!(actual, false);
    }

    #[test]
    fn test_clean_string_cases() {
        // Given
        let input = "  Correct Ans?,!.:;Wer  ".to_string();
        let expected = "correct answer".to_string();

        // When
        let actual = clean_string(&input);

        // Then
        assert_eq!(actual, expected);
    }

}