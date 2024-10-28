use eframe::egui::{self, RichText};

use super::PaneComponent;




pub struct HomeScreen {
    pub name: String,
}

impl Default for HomeScreen {
    fn default() -> Self {
        Self {
            name: "HomeScreen".to_owned(),
        }
    }
}

impl PaneComponent for HomeScreen {
    fn ui(&mut self, ui: &mut egui::Ui)  {
        ui.label(
            RichText::new("Home")
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

    fn close(&mut self) -> bool {
        false
    }
}