use eframe::egui::{self, RichText};

pub struct EditorScreen {
    pub name: String,
}

impl super::communication::PaneComponent for EditorScreen {
    fn ui(&self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("Editor")
                .heading()
                .color(egui::Color32::from_rgb(0, 255, 0)),
        );
        ui.separator();
        ui.label(self.name.clone());
    }
}
