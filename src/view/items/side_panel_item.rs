use eframe::egui::{self, Image, Response, Vec2};

use crate::view::screens::AnchorScreen;

#[derive(Clone, Debug)]
pub struct SidePanelItem<'a> {
    pub id: AnchorScreen,
    pub name: String,
    pub selected: bool,
    pub icon: egui::Image<'a>,
}

impl<'a> SidePanelItem<'a> {
    pub fn new(name: &str) -> Self {
        let file_path = "../../../assets/editor.png";
        let image = Image::from_uri(format!("file://{file_path}"));
        Self {
            id : AnchorScreen::Undefined,
            name: name.to_owned(),
            selected: false,
            icon: image,
        }
    }

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

        let response = ui.add(rich_button);
        response
    }

    pub fn show_with_text(&mut self, ui: &mut egui::Ui) -> Response {
        let text = egui::RichText::new(self.name.clone()).heading();

        let image = Some(self.icon.clone());

        let rich_button = egui::Button::opt_image_and_text(image, Some(text.into()))
            .fill(ui.style().visuals.extreme_bg_color) 
            .selected(self.selected)
            .wrap();
        // let rich_button = egui::widgets::Button::new( text);

        let vec: Vec2 = Vec2 {
            x: ui.available_width(),
            y: ui.spacing().interact_size.y,
        };

        // let response = ui.add_sized(ui.available_size(), rich_button);
        let response = ui.add_sized(vec, rich_button);

        response
    }
}
