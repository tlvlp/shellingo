use std::cell::RefCell;
use std::collections::{BTreeMap};
use ratatui_widgets::list::ListState;
use std::error::Error;
use std::ops::Not;
use std::rc::Rc;
use rand::seq::SliceRandom;
use ratatui_widgets::scrollbar::ScrollbarState;
use ratatui_widgets::table::TableState;
use strum::{EnumIter, EnumMessage, VariantArray};
use tui_input::Input;
use shellingo_core::practice;
use shellingo_core::question::Question;
use crate::question_parser::{collect_groups_from_multiple_paths, get_paths_from, read_all_questions_from_all_paths, QuestionGroup};

#[derive(Debug, Clone)]
pub enum AppPhase {
    Setup,
    Practice,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum UiComponent {
    GroupSelector,
    QuestionSelector,
    PracticeControls,
    PracticeMain,
    ExitPopup,
}

#[derive(EnumIter, EnumMessage,     VariantArray)]
pub enum PracticeControlOptions {
    #[strum(message="End Practice")]
    EndPractice,
    #[strum(message="Reset Stats")]
    ResetStats,
    #[strum(message="Try All")]
    TryAll,
    #[strum(message="Try Hardest 5")]
    TryHardest5,
    #[strum(message="Try Hardest 10")]
    TryHardest10,
}

#[derive(Debug)]
pub struct AppState {
    active_component: UiComponent,
    last_active_component: UiComponent,

    // Setup
    pub questions_by_groups: BTreeMap<String, QuestionGroup>,
    pub group_names_by_indices: BTreeMap<usize, String>,
    pub question_group_list_state: ListState,
    pub question_group_list_scrollbar_state: ScrollbarState,
    pub question_table_state: TableState,
    pub question_table_scrollbar_state: ScrollbarState,

    // Practice
    pub practice_controls_list_state: ListState,
    pub active_questions: Vec<Rc<RefCell<Question>>>,
    pub round_questions: Vec<Rc<RefCell<Question>>>,
    pub current_question_index: usize,
    pub answer_input: Input,
}

impl AppState {
    pub fn new(args: Vec<String>) -> Self {
        let paths_from_program_args = get_paths_from(args);

        // Workaround to map the indices of groups,
        // as RataTUI's List widget implementation can only return the index of a selected group.
        // BTreeMaps guarantee the fix order of groups in the main map to match the index map.
        let (questions_by_groups, group_names_by_indices) =
            collect_groups_from_multiple_paths(paths_from_program_args);

        let mut app = Self {
            active_component: UiComponent::GroupSelector,
            last_active_component: UiComponent::GroupSelector,

            // Setup
            questions_by_groups,
            group_names_by_indices,
            question_group_list_state: ListState::default(),
            question_group_list_scrollbar_state: ScrollbarState::default(),
            question_table_state: TableState::default(),
            question_table_scrollbar_state: ScrollbarState::default(),

            // Practice
            practice_controls_list_state: ListState::default(),
            active_questions: vec![],
            round_questions: vec![],
            current_question_index: 0,
            answer_input: Input::default(),
        };

        app.question_group_list_state.select_first();
        app.question_table_state.select_first();
        app.practice_controls_list_state.select_first();
        app
    }

    pub fn get_app_phase_for_active_component(&self) -> AppPhase {
        self.get_app_phase_for_component(&self.active_component)
    }

    fn get_app_phase_for_component(&self, component: &UiComponent) -> AppPhase {
        match component {
            UiComponent::GroupSelector | UiComponent::QuestionSelector => AppPhase::Setup,
            UiComponent::PracticeControls | UiComponent::PracticeMain => AppPhase::Practice,
            UiComponent::ExitPopup => {
                // Defined by the component the popup was opened from.
                self.get_app_phase_for_component(&self.last_active_component)
            },
        }
    }

    pub fn set_active_component(&mut self, component: UiComponent) {
        self.last_active_component = self.active_component.clone();
        self.active_component = component;
    }

    pub fn get_active_component(&mut self) -> UiComponent {
        self.active_component.clone()
    }

    pub fn setup_next_group(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_group_list_state.select_next();
        Ok(())
    }

    pub fn setup_previous_group(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_group_list_state.select_previous();
        Ok(())
    }

    pub fn setup_toggle_group_active_status_and_load_questions(&mut self) -> Result<(), Box<dyn Error>> {
        let selected_group_name = self.setup_get_selected_group_name().clone();
        let selected_group_op = self.questions_by_groups.get_mut(&selected_group_name);
        if selected_group_op.is_none() { return Ok(()) } //TODO: Proper error message for empty list.

        let selected_group = selected_group_op.unwrap();
        // Toggle active state
        selected_group.is_active = selected_group.is_active.not();

        if selected_group.is_active {
            // load questions
            let mut questions = read_all_questions_from_all_paths(&selected_group.paths);
            selected_group.questions.append(&mut questions);
        } else {
            // clear questions
            selected_group.questions.clear();
        }
        Ok(())
    }

    pub fn setup_get_questions_for_selected_group(&mut self) -> Vec<Rc<RefCell<Question>>> {
        let selected_group_name = self.setup_get_selected_group_name().clone();
        let group_op = self.questions_by_groups.get_mut(&selected_group_name);
        if group_op.is_none() { return vec![] }
        let group = group_op.unwrap();
        if group.is_active {
            group.questions.iter().cloned().collect() //note: Rc::clone() points to the same object
        } else {
            vec![]
        }
    }

    fn setup_get_selected_group_name(&mut self) -> &String {
        let selected_group_pos = self.question_group_list_state.selected().unwrap_or(0);
        self.group_names_by_indices.get(&selected_group_pos)
            .expect(format!("Can't find selected group at position {selected_group_pos}").as_str())
    }

    pub fn setup_previous_question(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_table_state.select_previous();
        Ok(())
    }

    pub fn setup_next_question(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_table_state.select_next();
        Ok(())
    }

    pub fn setup_toggle_panes(&mut self) -> Result<(), Box<dyn Error>> {
        if self.active_component == UiComponent::GroupSelector {
            self.set_active_component(UiComponent::QuestionSelector);
            if self.question_table_state.selected().is_none() {
            self.question_table_state.select_first();
            }
        } else {
            self.set_active_component(UiComponent::GroupSelector);
        }
        Ok(())
    }

    pub fn setup_navigate_to_practice(&mut self) -> Result<(), Box<dyn Error>> {
        self.active_questions = self.practice_get_all_active_questions();
        self.round_questions = self.active_questions.clone();
        self.practice_shuffle_questions();
        self.set_active_component(UiComponent::PracticeControls);
        Ok(())
    }

    pub fn practice_navigate_to_setup(&mut self) -> Result<(), Box<dyn Error>> {
        self.set_active_component(UiComponent::GroupSelector);
        Ok(())
    }

    pub fn practice_toggle_panes(&mut self) -> Result<(), Box<dyn Error>> {
        if self.active_component == UiComponent::PracticeControls {
            self.set_active_component(UiComponent::PracticeMain);
            //TODO select input component
        } else {
            self.set_active_component(UiComponent::PracticeControls);
        }
        Ok(())
    }

    pub fn practice_select_previous_menu_item(&mut self) -> Result<(), Box<dyn Error>> {
        self.practice_controls_list_state.select_previous();
        Ok(())
    }

    pub fn practice_select_next_menu_item(&mut self) -> Result<(), Box<dyn Error>> {
        self.practice_controls_list_state.select_next();
        Ok(())
    }

    fn practice_reset_round_question_filters_and_stats(&mut self) -> Result<(), Box<dyn Error>> {
        self.practice_reset_round_question_filters()?;
        self.round_questions.iter()
            .for_each(|question| question.borrow_mut().reset_round_stats());
        Ok(())
    }

    fn practice_filter_data_to_hardest_in_round(&mut self, limit: usize) -> Result<(), Box<dyn Error>> {
        self.round_questions = practice::get_hardest_questions_in_round(&self.active_questions, limit);
        self.practice_shuffle_questions();
        Ok(())
    }

    fn practice_reset_round_question_filters(&mut self) -> Result<(), Box<dyn Error>> {
        self.round_questions = self.active_questions.clone();
        self.practice_shuffle_questions();
        Ok(())
    }

    fn practice_get_all_active_questions(&mut self) -> Vec<Rc<RefCell<Question>>> {
        self.questions_by_groups.values()
            .filter(|group| group.is_active)
            .flat_map(|group| group.questions.iter().cloned())
            .collect()
    }

    pub fn practice_activate_selected_control(&mut self) -> Result<(), Box<dyn Error>> {
        let selected_index = self.practice_controls_list_state.selected()
            .unwrap_or(0);
        match PracticeControlOptions::VARIANTS[selected_index] {
            PracticeControlOptions::EndPractice => self.practice_navigate_to_setup(),
            PracticeControlOptions::ResetStats => self.practice_reset_round_question_filters_and_stats(),
            PracticeControlOptions::TryHardest5 => self.practice_filter_data_to_hardest_in_round(5),
            PracticeControlOptions::TryHardest10 => self.practice_filter_data_to_hardest_in_round(10),
            PracticeControlOptions::TryAll => self.practice_reset_round_question_filters(),
        }
    }

    pub fn practice_set_next_question_in_round(&mut self) -> Result<(), Box<dyn Error>>  {
        self.current_question_index += 1;
        if self.current_question_index.ge(&self.round_questions.len()) {
            self.practice_shuffle_questions();
        }
        Ok(())
    }

    fn practice_shuffle_questions(&mut self) {
        self.current_question_index = 0;
        self.round_questions.shuffle(&mut rand::rng());
    }

    pub fn practice_get_current_question_in_round(&mut self) -> Rc<RefCell<Question>> {
        self.round_questions.get(self.current_question_index).unwrap().clone()
    }

    pub fn practice_get_round_status_string(&mut self) -> String {
        format!("{}/{}", self.current_question_index + 1, self.round_questions.len())
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
