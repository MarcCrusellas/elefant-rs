use edit_screen::EditorScreen;
use eframe::egui;
use home_screen::HomeScreen;
use settings_screen::SettingsScreen;

use crate::utils::{Utils, Wrapped};

pub mod edit_screen;
pub mod home_screen;
pub mod settings_screen;

pub trait PaneComponent {
    
    fn ui(&self, ui: &mut egui::Ui);
    fn tab_title(&self) -> String;
}

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
    pub fn get_default_screen(&self) -> Wrapped<dyn PaneComponent> {
        match self {
            AnchorScreen::Home => Utils::get_wrapped(HomeScreen::default()),
            AnchorScreen::Editor => Utils::get_wrapped(EditorScreen::default()),
            AnchorScreen::Settings => Utils::get_wrapped(SettingsScreen::default()),
            AnchorScreen::Undefined => todo!(),
        }
        
    }
}
