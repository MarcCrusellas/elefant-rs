use eframe::egui::{self, RichText};

use super::PaneComponent;

pub struct SettingsScreen {
    pub name: String,
}

impl Default for SettingsScreen {
    fn default() -> Self {
        Self {
            name: "SettingsScreen".to_owned(),
        }
    }
}

impl PaneComponent for SettingsScreen {
    fn ui(&mut self, ui: &mut egui::Ui)  {
        ui.label(
            RichText::new("Settings")
                .heading()
                .color(egui::Color32::from_rgb(255, 0, 0)),
        );
        ui.separator();
        ui.label(self.name.clone());
    }

    fn tab_title(&mut self) -> String {
        self.name.clone()
    }

    fn save(&mut self) {}
}