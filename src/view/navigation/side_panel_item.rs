use eframe::egui::{self, Image, Response, Vec2};

use crate::view::screens::AnchorScreen;

#[derive(Clone, Debug)]
pub struct SidePanelItem<'a> {
    pub id: AnchorScreen,
    pub name: String,
    pub selected: bool,
    pub icon: Image<'a>,
}

impl<'a> SidePanelItem<'a> {
    pub fn show(&mut self, ui: &mut egui::Ui, expanded: bool, is_selected: bool) -> Response {
        self.selected = is_selected;
        if expanded {
            self.show_with_text(ui)
        } else {
            self.show_mini(ui)
        }
    }

    pub fn show_mini(&mut self, ui: &mut egui::Ui) -> Response {
        let rich_button = egui::ImageButton::new(self.icon.clone()).selected(self.selected);

        
        ui.add(rich_button)
    }

    pub fn show_with_text(&mut self, ui: &mut egui::Ui) -> Response {
        let text = egui::RichText::new(self.name.clone()).heading();

        let image = Some(self.icon.clone());

        let rich_button = egui::Button::opt_image_and_text(image, Some(text.into()))
            .fill(ui.style().visuals.extreme_bg_color) 
            .selected(self.selected)
            .wrap();

        let vec: Vec2 = Vec2 {
            x: ui.available_width(),
            y: ui.spacing().interact_size.y,
        };


        ui.add_sized(vec, rich_button)
    }
}
