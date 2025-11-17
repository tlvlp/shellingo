use regex::{Regex};
use shellingo_core::question::Question;
use std::{collections::{HashMap}, env, fs::{self}, path::PathBuf, sync::LazyLock};
use std::collections::{BTreeMap};
use walkdir::{DirEntry, Error, WalkDir};
use crate::app::QuestionGroupDetails;

static MULTIPLE_WHITESPACES_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());

/// Returns the paths passed in as commandline arguments or the current working directory if there was none
pub fn get_paths_from(args: Vec<String>) -> Vec<PathBuf> {
    let paths: Vec<PathBuf> = args.into_iter()
        .map(PathBuf::from)
        .collect();
    if paths.is_empty() { vec![env::current_dir().unwrap()] } else { paths }
}

pub fn collect_all_groups_from(paths: Vec<PathBuf>) -> BTreeMap<String, QuestionGroupDetails> {
    paths.into_iter()
        .flat_map(get_groups_from)
        .fold(BTreeMap::new(), |mut acc, map_entry| {
            let group = map_entry.0;
            let paths = map_entry.1;
            acc.entry(group)
                .or_insert(QuestionGroupDetails::default())
                .paths.extend(paths);
            acc
        })
}

fn get_groups_from(path: PathBuf) -> HashMap<String, Vec<PathBuf>> {
    get_all_files_under(path)
        .into_iter()
        .filter(|dir_entry| dir_entry.file_name().to_owned()
            .into_string()
            .unwrap_or("".to_string())
            .ends_with(".sll")
        )
        .fold(HashMap::new(), |mut acc, dir_entry| {
            let file_name = dir_entry.file_name()
                .to_owned()
                .into_string()
                .unwrap() // Already filtered
                .replace(".sll", "");
            let paths = acc.entry(file_name).or_insert(Vec::new());
            paths.push(PathBuf::from(dir_entry.into_path()));
            acc
        })
}

fn get_all_files_under(path: PathBuf) -> Vec<DirEntry> {
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

/// Returns all Questions under the provided path.
/// Takes both a single file or a directory and recursively parses all Questions under them.
pub fn read_all_questions_from(path: PathBuf) -> HashMap<String, Question> {
    get_all_files_under(path)
        .into_iter()
        .filter_map(read_file_to_string_or_skip_on_error)
        .flat_map(get_lines_from_string)
        .filter_map(parse_question_from_line)
        .fold(HashMap::new(), |map, new_question| {
            merge_questions(map, new_question)
        })
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
    if line.starts_with("#") {
        return None; // Skip comments. 
    };
    let split_q: Vec<&str> = line.split_terminator('|').collect();
    if split_q.len() != 2 {
        print!(
            "Error, skipping malformed question. Location:'{}' Line: '{}'",
            location, line
        );
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

fn merge_questions(
    mut map: HashMap<String, Question>,
    new_q: Question,
) -> HashMap<String, Question> {
    // Merges answers for the same question.
    map.entry(new_q.question.clone())
        .and_modify(|old_q| {
            old_q.solutions = old_q.solutions.union(&new_q.solutions).cloned().collect();
            old_q.locations = old_q.locations.union(&new_q.locations).cloned().collect();
        })
        .or_insert(new_q);
    map
}

#[cfg(test)]
mod tests {
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
        let mut expected = BTreeMap::new();
        expected.insert("f1_q1".to_string(), QuestionGroupDetails {
            questions: vec![],
            paths: vec![PathBuf::from("tests/fixtures/nested_with_mixed_files/f1/f1_q1.sll")],
            is_active: false,
        });
        expected.insert("f0_q1".to_string(), QuestionGroupDetails {
            questions: vec![],
            paths: vec![PathBuf::from("tests/fixtures/nested_with_mixed_files/f0_q1.sll")],
            is_active: false,
        });

        // When
        let actual = collect_all_groups_from(paths);

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
        let mut expected = BTreeMap::new();

        expected.insert("f1_q1".to_string(), QuestionGroupDetails {
            questions: vec![],
            paths: vec![PathBuf::from("tests/fixtures/duplicate_groups/nested_1/f1/f1_q1.sll"), PathBuf::from("tests/fixtures/duplicate_groups/nested_2/f1/f1_q1.sll")],
            is_active: false,
        });
        expected.insert("f0_q1".to_string(), QuestionGroupDetails {
            questions: vec![],
            paths: vec![PathBuf::from("tests/fixtures/duplicate_groups/nested_1/f0_q1.sll"), PathBuf::from("tests/fixtures/duplicate_groups/nested_2/f0_q1.sll")],
            is_active: false,
        });

        // When
        let actual = collect_all_groups_from(paths);

        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn comments_are_skipped() {
        // Given
        let path = PathBuf::from("tests/fixtures/comment");
        // When
        let question_map = read_all_questions_from(path);
        // Then
        assert_eq!(question_map.len(), 1);
    }

    #[test]
    fn same_question_with_different_answers_in_multiple_files_collected_to_a_single_question() {
        // Given
        let path = PathBuf::from("tests/fixtures/collect");
        let question_key = "question".to_owned();

        let answer_1 = "f0_q1 answer";
        let answer_2 = "f0_q2 answer";
        let answer_3 = "f1_q1 answer";

        let location_1 = "tests/fixtures/collect/f1/f1_q1.sll";
        let location_2 = "tests/fixtures/collect/f0_q1.sll";
        let location_3 = "tests/fixtures/collect/f0_q2.sll";

        // When
        let question_map = read_all_questions_from(path);

        // Then
        let solutions = &question_map.get(&question_key).unwrap().solutions;
        let locations = &question_map.get(&question_key).unwrap().locations;

        assert_eq!(question_map.len(), 1, "All 3 lines from 3 different files merged as one due to their matching question.");
        assert_eq!(solutions.len(), 3, "All 3 answers are kept for the question.");
        assert_eq!(locations.len(), 3, "All 3 locations are kept for the question.");

        assert!(solutions.contains(answer_1), "The expected answer 1 is present");
        assert!(solutions.contains(answer_2), "The expected answer 2 is present");
        assert!(solutions.contains(answer_3), "The expected answer 3 is present");

        assert!(locations.contains(location_1), "The expected location 1 is present");
        assert!(locations.contains(location_2), "The expected location 2 is present");
        assert!(locations.contains(location_3), "The expected location 3 is present");
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
