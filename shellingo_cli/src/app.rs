use ratatui::prelude::Span;
use ratatui_widgets::list::{ListItem, ListState};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::ops::Not;
use std::path::PathBuf;
use shellingo_core::question::Question;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumIter};
use crate::question_parser::{collect_all_groups_from, get_paths_from};

/// The component that has the focus / is currently active and receives key inputs.
#[derive(Display, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum UiFocus {
    Menu,
    Body,
    Popup,
}

/// Menu tabs
/// The sequence of the enum items determine their positions in the UI Menu
#[derive(Display, EnumCount, EnumIter, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum UiMenuItem {
    Questions,
    Practice,
    Exit,
}

/// Screens of the Body
#[derive(Display, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum UiBodyItem {
    QuestionSelector,
    PracticeScreen,
    ExitPopup,
}

/// Popup drawn above the other components with an unchangeable focus.
#[derive(Display, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Popup {
    None,
    // QuestionEditor,
    ExitConfirmation,
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct QuestionGroupDetails {
    pub questions: Vec<Question>,
    pub paths: Vec<PathBuf>,
    pub is_selected: bool,
}

#[derive(Debug)]
pub struct AppState<'a> {
    // Pre-calculated constants
    pub menu_to_pos: HashMap<UiMenuItem, usize>,
    pub pos_to_menu: HashMap<usize, UiMenuItem>,
    pub menu_item_spans: Vec<Span<'a>>,
    // Variables
    pub active_menu: UiMenuItem,
    pub active_screen: UiBodyItem,
    pub active_popup: Popup,
    pub focused_component: UiFocus,
    pub questions_by_groups: BTreeMap<String, QuestionGroupDetails>,
    pub question_group_list_state: ListState,
    pub question_group_names: Vec<String>,
}


impl<'a> AppState<'a> {
    pub fn new(args: Vec<String>) -> Self {
        // Generate UI menu items from the enum
        let menu_item_spans = UiMenuItem::iter()
            .map(|mi| Cow::from(mi.to_string()))
            .map(Span::from)
            .collect::<Vec<_>>();

        // Auto-generate relations between UI elements defined by the sequence of the Enum items.
        let mut iter: usize = 0;
        let menu_to_pos: HashMap<UiMenuItem, usize> = UiMenuItem::iter()
            .map(|m| {
                let mapping = (m, iter);
                iter += 1;
                return mapping;
            })
            .collect();
        let pos_to_menu: HashMap<usize, UiMenuItem> = menu_to_pos
            .iter()
            .map(|(m, p)| (p.clone(), m.clone()))
            .collect();

        // Loaded question groups from paths passes as commandline arguments
        let paths = get_paths_from(args);
        let questions_by_groups = collect_all_groups_from(paths);
        let question_group_names = questions_by_groups.keys()
            .cloned()
            .collect::<Vec<_>>();

        let mut question_group_list_state = ListState::default();
        question_group_list_state.select_first();

        Self {
            // Menu mappings
            menu_item_spans,
            menu_to_pos,
            pos_to_menu,

            // Default App State
            active_menu: UiMenuItem::Questions,
            active_screen: UiBodyItem::QuestionSelector,
            active_popup: Popup::None,
            focused_component: UiFocus::Body,
            questions_by_groups,
            question_group_list_state,
            question_group_names
        }
    }

    pub fn select_next_menu(&mut self) -> Result<(), Box<dyn Error>> {
        self.select_menu_relative_to(|current_pos| current_pos + 1);
        Ok(())
    }

    pub fn select_prev_menu(&mut self) -> Result<(), Box<dyn Error>> {
        self.select_menu_relative_to(|current_pos| current_pos - 1);
        Ok(())
    }

    fn select_menu_relative_to(&mut self, pos_change: fn(i8) -> i8) {
        let current_pos = self.get_active_menu_position() as i8;
        let new_pos = self.get_valid_position(pos_change(current_pos));
        self.active_menu = self.get_menu_form_pos(&new_pos);
    }

    pub fn get_active_menu_position(&self) -> usize {
        self.menu_to_pos
            .get(&self.active_menu)
            .expect(
                format!(
                    "MenuItem missing position allocation: {}!",
                    self.active_menu.to_string()
                )
                .as_str(),
            )
            .clone()
    }

    fn get_menu_form_pos(&self, pos: &usize) -> UiMenuItem {
        self.pos_to_menu
            .get(pos)
            .expect(format!("Position is not allocated to MenuItem: {}!", pos).as_str())
            .clone()
    }

    fn get_valid_position(&self, new_pos: i8) -> usize {
        let last_pos = UiMenuItem::COUNT as i8 - 1;
        if new_pos > last_pos {
            return 0;
        }
        if new_pos < 0 {
            return last_pos as usize;
        }
        new_pos as usize
    }

    pub fn navigate_to_selected_menu(&mut self) -> Result<(), Box<dyn Error>> {
        if self.active_menu == UiMenuItem::Exit {
            return Err(Box::from("Exiting application."));
        }
        self.active_screen = Self::menu_to_screen(&self.active_menu);
        Ok(())
    }

    fn menu_to_screen(menu_item: &UiMenuItem) -> UiBodyItem {
        match menu_item {
            UiMenuItem::Questions => UiBodyItem::QuestionSelector,
            UiMenuItem::Practice => UiBodyItem::PracticeScreen,
            UiMenuItem::Exit => UiBodyItem::ExitPopup,
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

    pub fn toggle_group_item_selection(&mut self) -> Result<(), Box<dyn Error>> {
        let selected_pos = self.question_group_list_state.selected();
        if selected_pos.is_none() { self.question_group_list_state.select_first() }

        // FIXME: this assumes unchanged order between `question_group_names` and `question_group_list_state`.
        //        There must be a nicer way to do this.
        let selected_group_name = self.question_group_names.get(selected_pos.unwrap()).unwrap();
        let group_details = self.questions_by_groups.get_mut(selected_group_name).unwrap();
        group_details.is_selected = group_details.is_selected.not();
        Ok(())
    }

    /// Switches the focused component back and forth between the menu and the body
    pub fn switch_component_focus(&mut self) -> Result<(), Box<dyn Error>> {
        match self.focused_component {
            UiFocus::Menu => self.focused_component = UiFocus::Body,
            UiFocus::Body => self.focused_component = UiFocus::Menu,
            UiFocus::Popup => { /* Do nothing, popup actions should be submitted or cancelled */
            }
        }
        Ok(())
    }

    pub fn open_popup(&mut self, popup: Popup) -> Result<(), Box<dyn Error>> {
        self.focused_component = UiFocus::Popup;
        self.active_popup = popup;
        if popup == Popup::ExitConfirmation {
            // Todo implement actual exit confirmation popup
            return Err(Box::from("Exiting application."));
        }
        Ok(())
    }

    pub fn close_active_popup(&mut self) -> Result<(), Box<dyn Error>> {
        self.focused_component = UiFocus::Body;
        self.active_popup = Popup::None;
        Ok(())
    }
}
