use rand::seq::SliceRandom;
use crate::question::Question;

pub const CLUE_REVEAL_PENALTY: u16 = 5;
pub const ANSWER_REVEAL_PENALTY: u16 = 10;

pub fn randomize_questions(questions: &Vec<Question>) -> Vec<&Question> {
    let mut question_refs = questions.iter().collect::<Vec<&Question>>();
    question_refs.shuffle(&mut rand::rng());
    question_refs
}

pub fn reveal_clue_for_penalty(question: &mut Question) -> String {
    question.increment_error_count(&CLUE_REVEAL_PENALTY);
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
    question.increment_error_count(&ANSWER_REVEAL_PENALTY);
    question.answers.iter()
        .cloned()
        .reduce(|a, b| format!("{a} or {b}"))
        .unwrap_or(format!("Cannot generate clue for the answer(s): '{:?}'", question.answers))
}


// TODO methods:
//  - fn reset_to_hardest_x <---- resetToHardest
//  ~~~~~~~~~
//  - fn clean_response_before_comparison
//      .strip()
//      .toLowerCase()
//      .replaceAll("[?,!.:;]", "");
//      .replaceAll("\\s{2,}", " ")
//  - fn evaluate_response




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
            question.get_question_stats().error_count_round == CLUE_REVEAL_PENALTY
                && question.get_question_stats().error_count_sum == CLUE_REVEAL_PENALTY
                && question.get_question_stats().correct_count_round == 0
                && question.get_question_stats().correct_count_sum == 0,
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
            question.get_question_stats().error_count_round == ANSWER_REVEAL_PENALTY
                && question.get_question_stats().error_count_sum == ANSWER_REVEAL_PENALTY
                && question.get_question_stats().correct_count_round == 0
                && question.get_question_stats().correct_count_sum == 0,
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

}