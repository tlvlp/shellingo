use std::cell::RefCell;
use std::collections::{BTreeMap};
use ratatui_widgets::list::ListState;
use std::error::Error;
use std::ops::Not;
use std::rc::Rc;
use ratatui_widgets::scrollbar::ScrollbarState;
use ratatui_widgets::table::TableState;
use strum::{EnumIter, EnumMessage, VariantArray};
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
    pub questions_by_groups: BTreeMap<String, QuestionGroup>,
    pub group_names_by_indices: BTreeMap<usize, String>,
    pub active_questions: Vec<Rc<RefCell<Question>>>,
    pub filtered_questions: Vec<Rc<RefCell<Question>>>,

    pub question_group_list_state: ListState,
    pub question_group_list_scrollbar_state: ScrollbarState,

    pub question_table_state: TableState,
    pub question_table_scrollbar_state: ScrollbarState,

    pub practice_controls_list_state: ListState,
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
            questions_by_groups,
            group_names_by_indices,
            active_questions: vec![],
            filtered_questions: vec![],
            question_group_list_state: ListState::default(),
            question_group_list_scrollbar_state: ScrollbarState::default(),
            question_table_state: TableState::default(),
            question_table_scrollbar_state: ScrollbarState::default(),
            practice_controls_list_state: ListState::default(),
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

    pub fn next_group(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_group_list_state.select_next();
        Ok(())
    }

    pub fn previous_group(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_group_list_state.select_previous();
        Ok(())
    }

    pub fn toggle_group_active_status_and_load_questions(&mut self) -> Result<(), Box<dyn Error>> {
        let selected_group_name = self.get_selected_group_name().clone();
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

    pub fn get_questions_for_selected_group(&mut self) -> Vec<Rc<RefCell<Question>>> {
        let selected_group_name = self.get_selected_group_name().clone();
        let group_op = self.questions_by_groups.get_mut(&selected_group_name);
        if group_op.is_none() { return vec![] }
        let group = group_op.unwrap();
        if group.is_active {
            group.questions.iter().cloned().collect() //note: Rc::clone() points to the same object
        } else {
            vec![]
        }
    }

    fn get_selected_group_name(&mut self) -> &String {
        let selected_group_pos = self.question_group_list_state.selected().unwrap_or(0);
        self.group_names_by_indices.get(&selected_group_pos)
            .expect(format!("Can't find selected group at position {selected_group_pos}").as_str())
    }

    pub fn previous_question(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_table_state.select_previous();
        Ok(())
    }

    pub fn next_question(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_table_state.select_next();
        Ok(())
    }

    pub fn toggle_setup_panes(&mut self) -> Result<(), Box<dyn Error>> {
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

    pub fn open_exit_popup(&mut self) -> Result<(), Box<dyn Error>> {
        self.set_active_component(UiComponent::ExitPopup);
        Ok(())
    }
    pub fn close_exit_popup(&mut self) -> Result<(), Box<dyn Error>> {
        self.set_active_component(self.last_active_component.clone());
        Ok(())
    }
    pub fn navigate_to_practice_main(&mut self) -> Result<(), Box<dyn Error>> {
        self.active_questions = self.get_all_active_questions();
        self.set_active_component(UiComponent::PracticeControls);
        Ok(())
    }

    pub fn navigate_to_setup(&mut self) -> Result<(), Box<dyn Error>> {
        self.set_active_component(UiComponent::GroupSelector);
        Ok(())
    }

    pub fn toggle_practice_panes(&mut self) -> Result<(), Box<dyn Error>> {
        if self.active_component == UiComponent::PracticeControls {
            self.set_active_component(UiComponent::PracticeMain);
            //TODO select input component
        } else {
            self.set_active_component(UiComponent::PracticeControls);
        }
        Ok(())
    }

    pub fn select_previous_practice_control_menu_item(&mut self) -> Result<(), Box<dyn Error>> {
        self.practice_controls_list_state.select_previous();
        Ok(())
    }

    pub fn select_next_practice_control_menu_item(&mut self) -> Result<(), Box<dyn Error>> {
        self.practice_controls_list_state.select_next();
        Ok(())
    }

    fn reset_practice_round(&mut self) -> Result<(), Box<dyn Error>> {
       todo!("reset the round counters on all the questions + use the originally selected list without filters")
    }

    fn filter_practice_data_to_hardest_in_round(&mut self, limit: usize) -> Result<(), Box<dyn Error>> {
        self.active_questions = practice::get_hardest_questions_in_round(&self.active_questions, limit);
        Ok(())
    }

    fn get_all_active_questions(&mut self) -> Vec<Rc<RefCell<Question>>> {
        self.questions_by_groups.values()
            .filter(|group| group.is_active)
            .flat_map(|group| group.questions.iter().cloned())
            .collect()
    }

    pub fn activate_selected_practice_control(&mut self) -> Result<(), Box<dyn Error>> {
        let selected_index = self.practice_controls_list_state.selected()
            .unwrap_or(0);
        match PracticeControlOptions::VARIANTS[selected_index] {
            PracticeControlOptions::EndPractice => self.navigate_to_setup(),
            PracticeControlOptions::ResetStats => self.reset_practice_round(),
            PracticeControlOptions::TryHardest5 => self.filter_practice_data_to_hardest_in_round(5),
            PracticeControlOptions::TryHardest10 => self.filter_practice_data_to_hardest_in_round(10),
            PracticeControlOptions::TryAll => todo!(),
        }
    }


    pub fn exit_app(&mut self) -> Result<(), Box<dyn Error>> {
        Err(Box::from("Exiting application."))
    }


}
