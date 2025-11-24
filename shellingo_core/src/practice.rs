use std::cell::RefCell;
use std::rc::Rc;
use std::sync::LazyLock;
use rand::seq::SliceRandom;
use regex::Regex;
use crate::question::Question;

pub const CLUE_REVEAL_PENALTY: u16 = 5;
pub const ANSWER_REVEAL_PENALTY: u16 = 10;
static REGEX_MULTIPLE_SPACES: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());
static REGEX_SYMBOLS_TO_REMOVE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[?,!.:;]+").unwrap());

pub fn randomize_questions(questions: &Vec<Question>) -> Vec<&Question> {
    let mut question_refs = questions.iter().collect::<Vec<&Question>>();
    question_refs.shuffle(&mut rand::rng());
    question_refs
}

pub fn reveal_clue_for_penalty(question: &mut Question) -> String {
    question.increment_error_count(CLUE_REVEAL_PENALTY);
    question.answers.iter()
        .map(|answer|
            answer.chars()
                .enumerate()
                // Mask every second character
                .map(|(index, character)| if index % 2 == 0 { character } else { '■' })
                .collect::<String>()
        )
        .reduce(|a, b| format!("{a} or {b}"))
        .unwrap_or(format!("Cannot generate clue for the answer(s): '{:?}'", question.answers))
}

pub fn reveal_answer_for_penalty(question: &mut Question) -> String {
    question.increment_error_count(ANSWER_REVEAL_PENALTY);
    question.answers.iter()
        .cloned()
        .reduce(|a, b| format!("{a} or {b}"))
        .unwrap_or(format!("Cannot generate clue for the answer(s): '{:?}'", question.answers))
}

pub fn get_hardest_questions_in_round(questions: &Vec<Rc<RefCell<Question>>>, limit: usize) -> Vec<Rc<RefCell<Question>>> {
    let mut refs = questions.iter().cloned().collect::<Vec<Rc<RefCell<Question>>>>();
    refs.sort_by(|a, b|
        // Reverse sort
        b.borrow().get_error_count_for_round()
            .cmp(&a.borrow().get_error_count_for_round())
    );
    refs.into_iter()
        .take(limit)
        .collect()
}

pub fn validate_attempt(attempt: &String, question: &mut Question) -> bool {
    let cleaned_attempt = clean_string(attempt);
    let is_success = question.answers.iter()
        .map(|answer| clean_string(answer) == cleaned_attempt)
        .reduce(|a, b| a || b)
        .unwrap_or(false);
    if is_success {
        question.increment_correct_count(1)
    } else {
        question.increment_error_count(1)
    }
    is_success
}

fn clean_string(response: &String) -> String {
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
    fn randomize_questions_should_return_vector_references_to_the_original_in_random_order() {
        // Given
        let questions = vec![
            Question::new("location_1".to_string(), "question_1".to_string(), "answer_1".to_string()),
            Question::new("location_2".to_string(), "question_2".to_string(), "answer_2".to_string()),
            Question::new("location_3".to_string(), "question_3".to_string(), "answer_3".to_string()),
            Question::new("location_4".to_string(), "question_4".to_string(), "answer_4".to_string()),
        ];

        // When
        let actual = randomize_questions(&questions);

        // Then
        questions.iter().for_each(|q| { assert!(actual.contains(&q)); });
        // Flaky, only for manual testing - sometimes the original sequence is returned.
        // assert!(
        //     questions[0] != *actual[0]
        //         || questions[1] != *actual[1]
        //         || questions[2] != *actual[2]
        //         || questions[3] != *actual[3]
        // )
    }

    #[test]
    fn reveal_clue_for_penalty_for_single_answer() {
        // Given
        let mut question = Question::new("location_1".to_string(), "question_1".to_string(), "answer_1".to_string());
        let expected = "a■s■e■_■".to_string();

        // When
        let actual = reveal_clue_for_penalty(&mut question);

        // Then
        assert_eq!(expected, actual);
        assert!(question.answers.contains("answer_1"), "The original answer remains unchanged");
        assert!(
            question.get_error_count_for_round() == CLUE_REVEAL_PENALTY
                && question.get_error_count_sum() == CLUE_REVEAL_PENALTY
                && question.get_correct_count_for_round() == 0
                && question.get_correct_count_sum() == 0,
            "The clue penalty is applied correctly"
        );
    }

    #[test]
    fn reveal_clue_for_penalty_with_multiple_answers() {
        // Given
        let mut question = Question::new("location_1".to_string(), "question_1".to_string(), "placeholder".to_string());
        question.answers = HashSet::from(["answer_one".to_string(), "answer_two".to_string()]);
        let expected = "a■s■e■_■n■ or a■s■e■_■w■".to_string();
        let expected_variant = "a■s■e■_■w■ or a■s■e■_■n■".to_string();

        // When
        let actual = reveal_clue_for_penalty(&mut question);

        // Then
        assert!(expected == actual || expected_variant == actual); // HashSet ordering can be random.
    }


    #[test]
    fn reveal_answer_for_penalty_for_single_answer() {
        // Given
        let mut question = Question::new("location_1".to_string(), "question_1".to_string(), "answer_1".to_string());
        let expected = "answer_1".to_string();

        // When
        let actual = reveal_answer_for_penalty(&mut question);

        // Then
        assert_eq!(expected, actual);
        assert!(
            question.get_error_count_for_round() == ANSWER_REVEAL_PENALTY
                && question.get_error_count_sum() == ANSWER_REVEAL_PENALTY
                && question.get_correct_count_for_round() == 0
                && question.get_correct_count_sum() == 0,
            "The clue penalty is applied correctly"
        );
    }

    #[test]
    fn reveal_answer_for_penalty_with_multiple_answers() {
        // Given
        let mut question = Question::new("location_1".to_string(), "question_1".to_string(), "placeholder".to_string());
        question.answers = HashSet::from(["answer_one".to_string(), "answer_two".to_string()]);
        let expected = "answer_one or answer_two".to_string();
        let expected_variant = "answer_two or answer_one".to_string();

        // When
        let actual = reveal_answer_for_penalty(&mut question);

        // Then
        assert!(expected == actual || expected_variant == actual); // HashSet ordering can be random.

    }

    // FIXME: Rc borrow something something.. mumble mumble
    // #[test]
    // fn test_get_hardest_questions_in_round() {
    //     // Given
    //     let q1 = Rc::new(RefCell::new(Question::new(String::new(), String::from("q1"), String::new())));
    //     let mut q2 = Rc::new(RefCell::new(Question::new(String::new(), String::from("q2"), String::new())));
    //     let mut q3 = Rc::new(RefCell::new(Question::new(String::new(), String::from("q3"), String::new())));
    //     let mut q4 = Rc::new(RefCell::new(Question::new(String::new(), String::from("q3"), String::new())));
    //     // Expected order: q4, q2, q3
    //     q2.get_mut().increment_error_count(5);
    //     q3.get_mut().increment_error_count(1);
    //     q4.get_mut().increment_error_count(10);
    //
    //     let limit = 3;
    //
    //     let questions = vec![q1.clone(), q2.clone(), q3.clone(), q4.clone()];
    //     let expected = vec![questions[3].clone(), questions[1].clone(), questions[2].clone()]; // Will drop q1, due to the limit.
    //
    //     // When
    //     let actual = get_hardest_questions_in_round(&questions, limit);
    //
    //     // Then
    //     assert_eq!(actual, expected);
    // }

    #[test]
    fn is_attempt_successful_matches_answer() {
        // Given
        let mut question = Question::new(String::new(), String::from("q1"), String::new());
        question.answers = HashSet::from(["answer one".to_string(), "answer two".to_string()]);
        let attempt = "answer one".to_string();

        // When
        let actual = validate_attempt(&attempt, &mut question);

        //Then
        assert_eq!(actual, true);
    }


    #[test]
    fn is_attempt_successful_no_answer_to_match() {
        // Given
        let mut question = Question::new(String::new(), String::from("q1"), String::new());
        question.answers = HashSet::from(["answer one".to_string(), "answer two".to_string()]);
        let attempt = "something else".to_string();

        // When
        let actual = validate_attempt(&attempt, &mut question);

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