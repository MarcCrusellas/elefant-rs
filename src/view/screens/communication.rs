use eframe::egui;

pub trait PaneComponent {
    fn ui(&self, ui: &mut egui::Ui);
}