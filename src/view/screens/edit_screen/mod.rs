use super::PaneComponent;
use eframe::egui::{self, Color32, Rect, RichText, Vec2};
pub mod json;
pub mod shape;

pub struct EditorScreen {
    pub name: String,
    pub rect: Option<Rect>,
}

impl Default for EditorScreen {
    fn default() -> Self {
        Self {
            name: "EditorScreen".to_owned(),
            rect: None,
        }
    }
}

impl PaneComponent for EditorScreen {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new("Editor")
                .heading()
                .color(egui::Color32::from_rgb(0, 255, 0)),
        );
        ui.separator();
        // egui::Frame::canvas(ui.style()).show(ui, |ui| {
        //     ui.set_height(ui.available_height() - 30.0);
        //     ui.set_width(ui.available_width() - 30.0);

        //     let (mut rect, response) =
        //         ui.allocate_exact_size(Vec2::new(30.0, 30.0), egui::Sense::drag());

        //     let drag = response.drag_delta();

        //     rect = rect.translate(drag);

        //     wrong way to draw a rectangle
        //     ui.painter().add(callback);
        // });

        // draw a dragable rectangle in the canvas
        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            ui.set_height(ui.available_height() - 30.0);
            ui.set_width(ui.available_width() - 30.0);

            // if self.rect == None {
            //     self.rect = Some(Rect::from_min_size(pos2(20.0, 20.0), vec2(30.0, 30.0)));
            // }

            let (rect, response) =
                ui.allocate_exact_size(Vec2::new(30.0, 30.0), egui::Sense::drag());
            let response: egui::Response =
                ui.allocate_rect(self.rect.unwrap(), egui::Sense::drag());

            let drag = response.drag_delta();

            // self.rect = self.rect.translate(drag);
            self.rect = Some(self.rect.unwrap().translate(drag));

            ui.painter()
                .rect_filled(self.rect.unwrap(), 0.0, Color32::from_rgb(255, 0, 0))
        });
    }

    fn tab_title(&mut self) -> String {
        self.name.clone()
    }

    fn save(&mut self) {}
}
