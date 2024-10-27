use eframe::egui::{self, RichText};

use super::PaneComponent;

pub struct EditorScreen {
    pub name: String,
}

impl Default for EditorScreen {
    fn default() -> Self {
        Self {
            name: "EditorScreen".to_owned(),
        }
    }
}

impl PaneComponent for EditorScreen {
    fn ui(&self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("Editor")
                .heading()
                .color(egui::Color32::from_rgb(0, 255, 0)),
        );
        ui.separator();
        ui.label(self.name.clone());
    }

    fn tab_title(&self) -> String {
        self.name.clone()
    }
}
