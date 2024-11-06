use egui_dock::DockState;

use crate::utils::Wrapped;

use super::screens::{AnchorScreen, PaneComponent};

pub type TabContentView = Wrapped<dyn PaneComponent + Send + Sync>;

pub struct State {
    pub is_window_open: bool,
    pub selected_item: AnchorScreen,
    pub side_panel_expanded: bool,
    pub tree: DockState<TabContentView>,
    pub counter: usize,
}

impl Default for State {
    fn default() -> Self {
        // let home_screen = HomeScreen::default();
        let tree: DockState<TabContentView> = DockState::new(vec![]);

        Self {
            is_window_open: false,
            selected_item: AnchorScreen::Home,
            side_panel_expanded: false,
            tree,
            counter: 1,
        }
    }
}
