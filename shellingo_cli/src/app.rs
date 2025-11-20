use ratatui_widgets::list::ListState;
use std::error::Error;
use std::ops::Not;
use ratatui_widgets::scrollbar::ScrollbarState;
use ratatui_widgets::table::TableState;
use shellingo_core::question::Question;
use crate::question_parser::{collect_groups_from_multiple_paths, get_paths_from, read_all_questions_from_paths, QuestionGroupDetails};

#[derive(Debug, Clone)]
pub enum AppPhase {
    Setup,
    // Practice,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum UiComponent {
    GroupSelector,
    QuestionSelector,
    ExitPopup,
}

#[derive(Debug)]
pub struct AppState {
    active_component: UiComponent,
    last_active_component: UiComponent,
    pub question_groups: Vec<QuestionGroupDetails>,
    pub question_group_list_state: ListState,
    pub question_group_list_scrollbar_state: ScrollbarState,
    pub question_table_state: TableState,
    pub question_table_scrollbar_state: ScrollbarState,
}

impl AppState {
    pub fn new(args: Vec<String>) -> Self {
        let paths_from_program_args = get_paths_from(args);

        let mut app = Self {
            active_component: UiComponent::GroupSelector,
            last_active_component: UiComponent::GroupSelector,
            question_groups: collect_groups_from_multiple_paths(paths_from_program_args),
            question_group_list_state: ListState::default(),
            question_group_list_scrollbar_state: ScrollbarState::default(),
            question_table_state: TableState::default(),
            question_table_scrollbar_state: ScrollbarState::default(),
        };

        app.question_group_list_state.select_first();
        app.question_table_state.select_first();
        app
    }

    pub fn get_app_phase_for_active_component(&self) -> AppPhase {
        self.get_app_phase_for_component(&self.active_component)
    }

    fn get_app_phase_for_component(&self, component: &UiComponent) -> AppPhase {
        match component {
            UiComponent::GroupSelector => {AppPhase::Setup}
            UiComponent::QuestionSelector => {AppPhase::Setup}
            UiComponent::ExitPopup => {
                self.get_app_phase_for_component(&self.last_active_component)
            }
        }
    }

    pub fn set_active_component(&mut self, component: UiComponent) {
        self.last_active_component = self.active_component.clone();
        self.active_component = component;
    }

    pub fn get_active_component(&mut self) -> UiComponent {
        self.active_component.clone()
    }

    pub fn next_group(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_group_list_state.select_next();
        self.question_group_list_scrollbar_state.next();
        Ok(())
    }

    pub fn previous_group(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_group_list_state.select_previous();
        self.question_group_list_scrollbar_state.prev();
        Ok(())
    }

    pub fn toggle_group_active_and_load_questions(&mut self) -> Result<(), Box<dyn Error>> {
        let selected_group = self.get_selected_group();
        // Toggle active state
        selected_group.is_active = selected_group.is_active.not();

        if selected_group.is_active {
            // load questions
            selected_group.questions = read_all_questions_from_paths(selected_group.paths.clone());
        } else {
            // clear questions
            selected_group.questions = vec![]
        }
        Ok(())
    }

    pub fn get_questions_for_selected_group(&mut self) -> Vec<Question> {
        let selected = self.get_selected_group();
        if selected.is_active {
            selected.questions.clone()
        } else {
            vec![]
        }
    }

    fn get_selected_group(&mut self) -> &mut QuestionGroupDetails {
        let selected_group_pos = self.question_group_list_state.selected().unwrap_or(0);
        self.question_groups.get_mut(selected_group_pos).unwrap()
    }

    pub fn previous_question(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_table_state.select_previous();
        self.question_table_scrollbar_state.prev();
        Ok(())
    }

    pub fn next_question(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_table_state.select_next();
        self.question_table_scrollbar_state.next();
        Ok(())
    }

    pub fn toggle_group_and_question_selectors(&mut self) -> Result<(), Box<dyn Error>> {
        if self.active_component == UiComponent::GroupSelector {
            self.set_active_component(UiComponent::QuestionSelector);
            self.question_table_state.select_first();
            self.question_table_scrollbar_state.first();
        } else {
            self.set_active_component(UiComponent::GroupSelector);
            self.question_table_state.select(None);
        }
        Ok(())
    }

    pub fn open_exit_popup(&mut self) -> Result<(), Box<dyn Error>> {
        self.set_active_component(UiComponent::ExitPopup);
        Ok(())
    }
    pub fn close_exit_popup(&mut self) -> Result<(), Box<dyn Error>> {
        self.set_active_component(self.last_active_component.clone());
        Ok(())
    }


    pub fn exit_app(&mut self) -> Result<(), Box<dyn Error>> {
        Err(Box::from("Exiting application."))
    }
}
