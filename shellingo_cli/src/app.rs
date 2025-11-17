use ratatui_widgets::list::{ListState};
use std::error::Error;
use std::ops::Not;
use std::path::PathBuf;
use ratatui_widgets::table::TableState;
use shellingo_core::question::Question;
use crate::question_parser::{collect_groups_from_multiple_paths, get_paths_from, read_all_questions_from_paths};

/// Screens of the Body
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum UiComponent {
    GroupSelector,
    QuestionSelector,
    ExitPopup,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QuestionGroupDetails {
    pub group_name: String,
    pub questions: Vec<Question>,
    pub paths: Vec<PathBuf>,
    pub is_active: bool,
}

#[derive(Debug)]
pub struct AppState {
    pub active_component: UiComponent,
    pub question_groups: Vec<QuestionGroupDetails>,
    pub question_group_list_state: ListState,
    pub question_table_state: TableState,
}

impl AppState {
    pub fn new(args: Vec<String>) -> Self {
        // Loaded question groups from paths passes as commandline arguments
        let paths = get_paths_from(args);
        let question_groups = collect_groups_from_multiple_paths(paths);

        let mut question_group_list_state = ListState::default();
        question_group_list_state.select_first();
        let mut question_table_state = TableState::default();
        question_table_state.select_first();

        Self {
            // Default App State
            active_component: UiComponent::GroupSelector,
            question_groups,
            question_group_list_state,
            question_table_state,
        }
    }

    pub fn next_group(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_group_list_state.select_next();
        Ok(())
    }

    pub fn previous_group(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_group_list_state.select_previous();
        Ok(())
    }

    pub fn toggle_group_active_and_load_questions(&mut self) -> Result<(), Box<dyn Error>> {
        let selected_group = self.get_selected_group();
        selected_group.is_active = selected_group.is_active.not();
        if selected_group.is_active {
            //load questions
            selected_group.questions = read_all_questions_from_paths(selected_group.paths.clone());
        }
        Ok(())
    }

    fn get_selected_group(&mut self) -> &mut QuestionGroupDetails {
        let selected_group_pos = self.question_group_list_state.selected().unwrap_or(0);
        self.question_groups.get_mut(selected_group_pos).unwrap()
    }

    pub fn open_exit_popup(&mut self) -> Result<(), Box<dyn Error>> {
        self.active_component = UiComponent::ExitPopup;
        // Todo implement actual exit confirmation popup
        Err(Box::from("Exiting application."))
    }
}
