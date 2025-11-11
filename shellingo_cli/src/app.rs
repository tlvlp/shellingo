use ratatui::prelude::Span;
use ratatui_widgets::list::{ListItem, ListState};
use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use shellingo_core::question::Question;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumIter};
use crate::question_parser::{collect_all_groups_from, get_paths_from};

/// The component that has the focus / is currently active and receives key inputs.
#[derive(Display, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum UiComponent {
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

#[derive(Debug)]
pub struct ParsedQuestionData<'a> {
    pub questions_by_group: BTreeMap<String, QuestionGroupDetails<'a>>,
    pub group_list_state: ListState,
}

#[derive(Debug)]
pub struct QuestionGroupDetails<'a> {
    pub group_list_item: ListItem<'a>,
    pub questions: Vec<Question>,
    pub is_selected: bool,
}

pub struct AppState<'a> {
    // Pre-calculated constants
    pub menu_to_pos: HashMap<UiMenuItem, usize>,
    pub pos_to_menu: HashMap<usize, UiMenuItem>,
    pub menu_item_spans: Vec<Span<'a>>,
    // Variables
    pub active_menu: UiMenuItem,
    pub active_screen: UiBodyItem,
    pub active_popup: Popup,
    pub focused_component: UiComponent,
    pub question_data: ParsedQuestionData<'a>,
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
        let question_groups = collect_all_groups_from(paths); //TODO: extend for on-demand question parsing

        let mut sorted_groups: Vec<String> = question_groups.keys().cloned().collect();
        sorted_groups.sort();
        let question_groups_for_list = sorted_groups.into_iter()
            .map(ListItem::new)
            .collect::<Vec<_>>();

        Self {
            // Menu mappings
            menu_item_spans,
            menu_to_pos,
            pos_to_menu,

            // Default App State
            active_menu: UiMenuItem::Questions,
            active_screen: UiBodyItem::QuestionSelector,
            active_popup: Popup::None,
            focused_component: UiComponent::Menu,
            question_data: ParsedQuestionData {
                questions_by_group: BTreeMap::new(),
                group_list_state: ListState::default(),
            },
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
        self.question_data.state.select_next();
        Ok(())
    }

    pub fn previous_group(&mut self) -> Result<(), Box<dyn Error>> {
        self.question_data.state.select_previous();
        Ok(())
    }

    pub fn select_group(&mut self) -> Result<(), Box<dyn Error>> {
        if self.question_data.items.is_empty() { return Ok(()) };
        // let selection = self.file_list.state.selected().unwrap_or(0);
        Ok(())
    }


    /// Switches the focused component back and forth between the menu and the body
    pub fn switch_component_focus(&mut self) -> Result<(), Box<dyn Error>> {
        match self.focused_component {
            UiComponent::Menu => self.focused_component = UiComponent::Body,
            UiComponent::Body => self.focused_component = UiComponent::Menu,
            UiComponent::Popup => { /* Do nothing, popup actions should be submitted or cancelled */
            }
        }
        Ok(())
    }

    pub fn open_popup(&mut self, popup: Popup) -> Result<(), Box<dyn Error>> {
        self.focused_component = UiComponent::Popup;
        self.active_popup = popup;
        if popup == Popup::ExitConfirmation {
            // Todo implement actual exit confirmation popup
            return Err(Box::from("Exiting application."));
        }
        Ok(())
    }

    pub fn close_active_popup(&mut self) -> Result<(), Box<dyn Error>> {
        self.focused_component = UiComponent::Body;
        self.active_popup = Popup::None;
        Ok(())
    }
}
