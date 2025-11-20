use regex::Regex;
use shellingo_core::question::Question;
use std::{env, fs::{self}, path::PathBuf, sync::LazyLock};
use std::collections::BTreeMap;
use walkdir::{DirEntry, Error, WalkDir};

static MULTIPLE_WHITESPACES_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QuestionGroupDetails {
    pub group_name: String,
    pub questions: Vec<Question>,
    pub paths: Vec<PathBuf>,
    pub is_active: bool,
}

/// Returns the paths passed in as commandline arguments or the current working directory if there was none
pub fn get_paths_from(args: Vec<String>) -> Vec<PathBuf> {
    let paths: Vec<PathBuf> = args.into_iter()
        .map(PathBuf::from)
        .collect();
    if paths.is_empty() { vec![env::current_dir().unwrap()] } else { paths }
}

pub fn collect_groups_from_multiple_paths(paths: Vec<PathBuf>) -> Vec<QuestionGroupDetails> {
    paths.into_iter()
        .flat_map(get_all_files_under_path)
        .filter(|dir_entry| dir_entry.file_name().to_owned()
            .into_string()
            .unwrap_or("".to_string())
            .ends_with(".sll")
        )
        // group Questions from files with matching names
        .fold(BTreeMap::new(), |mut acc, dir_entry| {
            let group_name = dir_entry.file_name()
                .to_owned()
                .into_string()
                .unwrap() // Already filtered
                .replace(".sll", "");

            let group_details = acc.entry(group_name.clone())
                .or_insert(QuestionGroupDetails::default());
            group_details.paths.push(PathBuf::from(dir_entry.into_path()));
            group_details.group_name = group_name;

            acc
        })
        .values()
        .cloned()
        .collect()

}

fn get_all_files_under_path(path: PathBuf) -> Vec<DirEntry> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(filter_readable_entries)
        .filter(filter_for_files)
        .collect()
}

fn filter_readable_entries(result: Result<DirEntry, Error>) -> Option<DirEntry> {
    match result {
        Ok(res) => Some(res),
        Err(e) => {
            print!("Error: Skipping unreadable directory entry with reason: {}", e);
            None
        }
    }
}

fn filter_for_files(dir_entry: &DirEntry) -> bool {
    !dir_entry.path().is_dir()
}

pub fn read_all_questions_from_paths(paths: Vec<PathBuf>) -> Vec<Question> {
    paths.iter()
        .cloned()
        .flat_map(get_all_files_under_path)
        .filter_map(read_file_to_string_or_skip_on_error)
        .flat_map(get_lines_from_string)
        .filter_map(parse_question_from_line)
        .fold(BTreeMap::new(), |mut acc, new_question| {
            let old_question = acc.entry(new_question.question.clone())
                .or_insert(new_question.clone());
            old_question.solutions = old_question.solutions.union(&new_question.solutions).cloned().collect();
            old_question.locations = old_question.locations.union(&new_question.locations).cloned().collect();
            acc
        })
        .values()
        .cloned()
        .collect()
}

struct ProcessingStep<T> {
    result: T,
    path: String,
}

fn read_file_to_string_or_skip_on_error(file: DirEntry) -> Option<ProcessingStep<String>> {
    let path = file.path();
    match fs::read_to_string(path) {
        Ok(file_str) => {
            Some(ProcessingStep {
                path: path.display().to_string(),
                result: file_str,
            })
        }
        Err(_) => {
            // Todo display a meaningful error
            println!("Error: Skipping unreadable file: {}", path.display());
            None
        }
    }
}

fn get_lines_from_string(contents: ProcessingStep<String>) -> Vec<ProcessingStep<String>> {
    let file_str = contents.result;
    file_str
        .lines()
        .map(str::to_owned)
        .map(|line| ProcessingStep::<String> {
            result: line,
            path: contents.path.to_owned(),
        })
        .collect()
}

fn parse_question_from_line(line_contents: ProcessingStep<String>) -> Option<Question> {
    let line = line_contents.result;
    let location = line_contents.path;
    if line.is_empty() || line.starts_with("#") || line.contains(r"^\s+$") {
        return None; // Skip empty or commented out lines.
    };
    let split_q: Vec<&str> = line.split_terminator('|').collect();
    if split_q.len() != 2 {
        // TODO: Meaningful error handling
        print!("Error, skipping malformed question. Location:'{location}' Line: '{line}'");
        return None;
    }
    let question = remove_extra_whitespaces(split_q[0]);
    let solution = remove_extra_whitespaces(split_q[1]);
    Some(Question::new(location, question, solution))
}

fn remove_extra_whitespaces(text: &str) -> String {
    MULTIPLE_WHITESPACES_REGEX
        .replace_all(text, " ")
        .trim_start()
        .trim_end()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::*;

    #[test]
    fn get_paths_from_args_test() {
        // Given
        let args = vec!["path1".to_string(), "path2".to_string()];
        let expected = vec![PathBuf::from("path1"), PathBuf::from("path2")];

        // When
        let actual = get_paths_from(args);

        // Then
        assert_eq!(expected, actual);
    }


    #[test]
    fn all_groups_are_collected_from_nested_subdirectories_with_mixed_file_types() {
        // Given
        let paths = vec![PathBuf::from("tests/fixtures/nested_with_mixed_files")];

        // When
        let expected = vec![
            QuestionGroupDetails {
                group_name: "f0_q1".to_string(),
                questions: vec![],
                paths: vec![PathBuf::from("tests/fixtures/nested_with_mixed_files/f0_q1.sll")],
                is_active: false,
            },
            QuestionGroupDetails {
                group_name: "f1_q1".to_string(),
                questions: vec![],
                paths: vec![PathBuf::from("tests/fixtures/nested_with_mixed_files/f1/f1_q1.sll")],
                is_active: false,
            },
        ];

        // When
        let actual = collect_groups_from_multiple_paths(paths);

        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn all_groups_are_collected_from_multiple_paths() {
        // Given
        let paths = vec![
            PathBuf::from("tests/fixtures/duplicate_groups/nested_1"),
            PathBuf::from("tests/fixtures/duplicate_groups/nested_2"),
        ];
        let expected = vec![
            QuestionGroupDetails {
                group_name: "f0_q1".to_string(),
                questions: vec![],
                paths: vec![PathBuf::from("tests/fixtures/duplicate_groups/nested_1/f0_q1.sll"), PathBuf::from("tests/fixtures/duplicate_groups/nested_2/f0_q1.sll")],
                is_active: false,
            },
            QuestionGroupDetails {
                group_name: "f1_q1".to_string(),
                questions: vec![],
                paths: vec![PathBuf::from("tests/fixtures/duplicate_groups/nested_1/f1/f1_q1.sll"), PathBuf::from("tests/fixtures/duplicate_groups/nested_2/f1/f1_q1.sll")],
                is_active: false,
            },
        ];

        // When
        let actual = collect_groups_from_multiple_paths(paths);

        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn comments_are_skipped() {
        // Given
        let path = vec![PathBuf::from("tests/fixtures/comment")];
        let expected = vec![Question::new(
            "tests/fixtures/comment".to_string(),
           "question".to_string(),
            "answer".to_string()
        )];

        // When
        let actual = read_all_questions_from_paths(path);

        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn same_question_with_different_answers_in_multiple_files_collected_to_a_single_question() {
        // Given
        let paths = vec![PathBuf::from("tests/fixtures/collect")];
        let expected = vec![
            Question {
                question: "question".to_string(),
                solutions: HashSet::from(["f0_q2 answer".to_string(), "f0_q1 answer".to_string(), "f1_q1 answer".to_string()]),
                locations: HashSet::from(["tests/fixtures/collect/f1/f1_q1.sll".to_string(), "tests/fixtures/collect/f0_q2.sll".to_string(), "tests/fixtures/collect/f0_q1.sll".to_string()]),
                correct_count_round: 0,
                error_count_round: 0,
                correct_count_sum: 0,
                error_count_sum: 0,
            }
        ];

        // When
        let actual = read_all_questions_from_paths(paths);

        // Then
        assert_eq!(expected, actual);
    }

    #[test]
    fn extra_whitespaces_are_removed() {
        // Given
        let input = "     my       question ";
        let expected = "my question".to_owned();
        // When
        let result = remove_extra_whitespaces(&input);
        // Then
        assert_eq!(expected, result);
    }
}

