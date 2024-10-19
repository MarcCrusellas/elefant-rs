use eframe::egui::{self, RichText};

pub struct HomeScreen {
    pub name: String,
}

impl super::communication::PaneComponent for HomeScreen {
    fn ui(&self, ui: &mut egui::Ui)  {
        ui.label(
            RichText::new("Home")
                .heading()
                .color(egui::Color32::from_rgb(255, 0, 0)),
        );
        ui.separator();
        ui.label(self.name.clone());
    }
}