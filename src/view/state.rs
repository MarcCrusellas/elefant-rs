use std::sync::{Arc, Mutex};


use egui_dock::DockState;

use crate::utils::Wrapped;

use super::screens::{ AnchorScreen, PaneComponent};

pub struct State {
    pub selected_item : AnchorScreen,
    pub side_panel_expanded: bool,
    pub tree: DockState<Wrapped<dyn PaneComponent>>,
    pub counter: usize,
}

impl Default for State {
    fn default() -> Self {
        // let home_screen = HomeScreen::default();
        let tree: DockState<Arc<Mutex<dyn PaneComponent>>> = DockState::new(vec![]);

        Self {
            selected_item: AnchorScreen::Home,
            side_panel_expanded: false,
            tree,
            counter: 1
        }
    }
}
