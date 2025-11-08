use ratatui::prelude::Span;
use ratatui_widgets::list::{ListItem, ListState};
use std::borrow::Cow;
use std::collections::HashMap;
use std::error::Error;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumIter};
use crate::question_parser::{get_all_question_groups_from, get_paths_from};
// use crate::question_parser::{get_paths_from};

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
pub enum MenuItem {
    Questions,
    Practice,
    Exit,
}

/// Screens of the Body
#[derive(Display, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Screen {
    QuestionSelector,
    PracticeScreen,
    ExitScreen,
}

/// Popup drawn above the other components with an unchangeable focus.
#[derive(Display, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Popup {
    None,
    // QuestionEditor,
    ExitConfirmation,
}

#[derive(Debug)]
pub struct FileList<'a> {
    pub items: Vec<ListItem<'a>>,
    pub state: ListState,
}

pub struct AppState<'a> {
    // Pre-calculated constants
    pub menu_to_pos: HashMap<MenuItem, usize>,
    pub pos_to_menu: HashMap<usize, MenuItem>,
    pub menu_item_spans: Vec<Span<'a>>,
    // Variables
    pub active_menu: MenuItem,
    pub active_screen: Screen,
    pub active_popup: Popup,
    pub focused_component: UiComponent,
    pub file_list: FileList<'a>,
}

impl<'a> AppState<'a> {
    pub fn new(args: Vec<String>) -> Self {
        // Generate UI menu items from the enum
        let menu_item_spans = MenuItem::iter()
            .map(|mi| Cow::from(mi.to_string()))
            .map(Span::from)
            .collect::<Vec<_>>();

        // Auto-generated relations between UI elements defined by the sequence of the Enum items.
        let mut iter: usize = 0;
        let menu_to_pos: HashMap<MenuItem, usize> = MenuItem::iter()
            .map(|m| {
                let mapping = (m, iter);
                iter += 1;
                return mapping;
            })
            .collect();
        let pos_to_menu: HashMap<usize, MenuItem> = menu_to_pos
            .iter()
            .map(|(m, p)| (p.clone(), m.clone()))
            .collect();

        // Statically loaded question groups from paths passes as commandline arguments
        let paths = get_paths_from(args);
        let question_groups = get_all_question_groups_from(paths);
        let question_groups_for_list = question_groups
            .iter()
            .map(|group| {ListItem::new(group.name.clone())})
            .collect::<Vec<_>>();

        Self {
            // Menu mappings
            menu_item_spans,
            menu_to_pos,
            pos_to_menu,

            // Default App State
            active_menu: MenuItem::Questions,
            active_screen: Screen::QuestionSelector,
            active_popup: Popup::None,
            focused_component: UiComponent::Menu,
            file_list: FileList {
                items: question_groups_for_list,
                state: ListState::default()
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

    fn get_menu_form_pos(&self, pos: &usize) -> MenuItem {
        self.pos_to_menu
            .get(pos)
            .expect(format!("Position is not allocated to MenuItem: {}!", pos).as_str())
            .clone()
    }

    fn get_valid_position(&self, new_pos: i8) -> usize {
        let last_pos = MenuItem::COUNT as i8 - 1;
        if new_pos > last_pos {
            return 0;
        }
        if new_pos < 0 {
            return last_pos as usize;
        }
        new_pos as usize
    }

    pub fn navigate_to_selected_menu(&mut self) -> Result<(), Box<dyn Error>> {
        if self.active_menu == MenuItem::Exit {
            return Err(Box::from("Exiting application."));
        }
        self.active_screen = Self::menu_to_screen(&self.active_menu);
        Ok(())
    }

    fn menu_to_screen(menu_item: &MenuItem) -> Screen {
        match menu_item {
            MenuItem::Questions => Screen::QuestionSelector,
            MenuItem::Practice => Screen::PracticeScreen,
            MenuItem::Exit => Screen::ExitScreen,
        }
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
