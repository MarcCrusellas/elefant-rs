use edit_screen::EditorScreen;
use eframe::egui;
use home_screen::HomeScreen;
use settings_screen::SettingsScreen;

use crate::utils::Utils;

use super::state::TabContentView;

pub mod edit_screen;
pub mod home_screen;
pub mod settings_screen;

pub trait PaneComponent {
    fn ui(&mut self, ui: &mut egui::Ui);
    
    fn tab_title(&mut self) -> String;

    fn save(&mut self);

    // If false is returned, the tab will not be closed
    fn close(&mut self) -> bool {
        true
    }
}

// unsafe impl Send for dyn PaneComponent {}
// unsafe impl Sync for dyn PaneComponent {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AnchorScreen {
    #[default]
    Undefined,
    Home,
    Editor,
    Settings
}

impl AnchorScreen {
    pub fn get_default_screen(&self) -> TabContentView {
        match self {
            AnchorScreen::Home => Utils::get_wrapped(HomeScreen::default()),
            AnchorScreen::Editor => Utils::get_wrapped(EditorScreen::default()),
            AnchorScreen::Settings => Utils::get_wrapped(SettingsScreen::default()),
            AnchorScreen::Undefined => todo!(),
        }
        
    }
}
