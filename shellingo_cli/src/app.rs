use std::collections::HashMap;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumIter};

#[derive(Display, EnumCount, EnumIter, Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum MenuItem {
    // : The sequence of the enum items determine their positions in the UI Menu
    Questions,
    Practice,
    Exit,
}

pub enum Screen {
    QuestionSelector,
    PracticeScreen,
    ExitScreen,
}

// pub enum Popup {
//     None,
//     QuestionEditor,
// }

pub struct AppState {
    pub menu_to_pos: HashMap<MenuItem, usize>,
    pub pos_to_menu: HashMap<usize, MenuItem>,
    pub active_menu: MenuItem,
    pub active_screen: Screen,
    // pub active_popup: Popup,
}

impl AppState {
    pub fn new() -> Self {
        // Auto-generated relations between UI elements defined by the sequence of the Enum items.
        let mut iter: usize = 0;
        let menu_to_pos: HashMap<MenuItem, usize> = MenuItem::iter()
            .map(|m| {
            let mapping = (m, iter.clone());
                iter += 1;
                return mapping
            }).collect();
        let pos_to_menu: HashMap<usize, MenuItem> = menu_to_pos.iter()
            .map(|(m, p)| (p.clone(), m.clone()))
            .collect();

        Self {
            // Menu mappings
            menu_to_pos,
            pos_to_menu,

            // Default App State
            active_menu: MenuItem::Questions,
            active_screen: Screen::QuestionSelector,
        }
    }

    pub fn navigate_to_next_menu(&mut self) {
        self.navigate_relative_to(|current_pos| current_pos + 1);
    }

    pub fn navigate_to_prev_menu(&mut self) {
        self.navigate_relative_to(|current_pos| current_pos - 1);
    }
    
    fn navigate_relative_to(&mut self, pos_change: fn(i8) -> i8) {
        let current_pos = self.get_active_menu_position() as i8;
        let new_pos = self.get_valid_position(pos_change(current_pos));
        let new_menu = self.get_menu_form_pos(&new_pos);
        self.navigate_to(new_menu); 
    }

    pub fn get_active_menu_position(&self) -> usize {
        self.menu_to_pos.get(&self.active_menu)
            .expect(format!("MenuItem missing position allocation: {}!", self.active_menu.to_string()).as_str())
            .clone()
    }

    fn get_menu_form_pos(&self, pos: &usize) -> MenuItem {
        self.pos_to_menu.get(pos)
            .expect(format!("Position is not allocated to MenuItem: {}!", pos).as_str())
            .clone()
    }

    fn get_valid_position(&self, new_pos: i8) -> usize {
        let last_pos = MenuItem::COUNT as i8  - 1;
        if (new_pos > last_pos) {
            return 0
        }
        if (new_pos < 0) {
            return last_pos as usize
        }
        new_pos as usize
    }

    pub fn navigate_to(&mut self, menu_item: MenuItem) {
        self.active_screen = Self::menu_to_screen(&menu_item);
        self.active_menu = menu_item;
    }

    fn menu_to_screen(menu_item: &MenuItem) -> Screen {
        match menu_item {
            MenuItem::Questions => Screen::QuestionSelector,
            MenuItem::Practice => Screen::PracticeScreen,
            MenuItem::Exit => Screen::ExitScreen,
        }
    }
}
