use bevy::prelude::Resource;
use eframe::egui::{self, Image};
use egui_dock::{DockArea, Style};

use crate::utils::Utils;

use super::{navigation::side_panel_item::SidePanelItem, screens::{home_screen::HomeScreen, AnchorScreen}, state, tree::TabViewer};

#[derive(Resource)]
pub struct MainEditor {
    pub text: String,
    pub state: state::State,
    pub side_panel_items: Vec<SidePanelItem<'static>>,
}

impl Default for MainEditor {
    fn default() -> Self {
        Self {
            text: "Hello World!".to_owned(),
            side_panel_items: vec![
                SidePanelItem {
                    id: AnchorScreen::Home,
                    name: "Home".to_owned(),
                    selected: false,
                    icon: Image::from_uri("file://../../../assets/home.png"),
                },
                SidePanelItem {
                    id: AnchorScreen::Editor,
                    name: "Editor".to_owned(),
                    selected: false,
                    icon: Image::from_uri(
                        "https://cdn-icons-png.flaticon.com/512/5273/5273716.png",
                    ),
                },
                SidePanelItem {
                    id: AnchorScreen::Settings,
                    name: "Settings".to_owned(),
                    selected: false,
                    icon: Image::from_uri("file://../../../assets/settings.png"),
                },
            ],
            state: state::State::default(),
        }
    }
}

impl eframe::App for MainEditor {

    

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_top_panel(ctx);
        self.show_side_panel(ctx);
        self.show_footer(ctx);
        self.show_central_panel(ctx);
        self.keybindings(ctx);

    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        std::process::exit(0);
    }
}

impl MainEditor {


    fn update(&mut self, ctx: &egui::Context) {
        self.show_top_panel(ctx);
        self.show_side_panel(ctx);
        self.show_footer(ctx);
        self.show_central_panel(ctx);
        self.keybindings(ctx);

    }

    pub fn show_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("Project", |ui| {
                        ui.button("New").clicked();
                        ui.button("Open").clicked();
                        ui.button("Save").clicked();
                        ui.separator();
                        if ui.button("Exit").clicked() {
                            std::process::exit(0);
                        }
                    });

                    ui.menu_button("Edit", |ui| {
                        ui.button("Add").clicked();
                        ui.separator();
                        ui.button("Undo").clicked();
                        ui.button("Redo").clicked();
                        ui.button("Cut").clicked();
                        ui.button("Copy").clicked();
                    });

                    ui.menu_button("View", |ui| {
                        ui.button("Fullscreen").clicked();
                        ui.button("Side Panel").clicked();
                    });
                    ui.spacing();

                    ui.menu_button("Help", |ui| {
                        ui.button("About").clicked();
                    });

                });
            });
        });
    }

    fn show_side_panel(&mut self, ctx: &egui::Context) {
        let collapsed_side_panel = egui::SidePanel::left("side_panel_collapsed")
            .default_width(50.0)
            .resizable(false);

        let expanded_side_panel = egui::SidePanel::left("side_panel_expanded")
            .default_width(250.0)
            .width_range(150.0..=400.0)
            .resizable(true);

        egui::SidePanel::show_animated_between(
            ctx,
            self.state.side_panel_expanded,
            collapsed_side_panel,
            expanded_side_panel,
            |ui, _| {
                ui.group(|ui| {
                    for item in self.side_panel_items.iter_mut() {
                        
                        if item
                            .show(
                                ui,
                                self.state.side_panel_expanded,
                                self.state.selected_item == item.id,
                            )
                            .clicked()
                        {
                            self.state.selected_item = item.id;
                            self.state.tree.push_to_focused_leaf(item.id.get_default_screen()
                            );
                        };

                        // if i != len - 1 {
                        //     ui.separator();
                        // }
                    }
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::BOTTOM), |ui| {
                    if ui.button("ddd").clicked() {
                        self.state.side_panel_expanded = !self.state.side_panel_expanded;
                    }
                });
            },
        );
    }

    fn show_footer(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.label("Footer");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                egui::widgets::global_theme_preference_switch(ui);
                {
                    let mut fullscreen = ui.input(|i| i.viewport().fullscreen.unwrap_or(false));
                    if ui
                        .checkbox(&mut fullscreen, "🗖 Fullscreen (F11)")
                        .on_hover_text("Fullscreen the window")
                        .changed()
                    {
                        ui.ctx()
                            .send_viewport_cmd(egui::ViewportCommand::Fullscreen(fullscreen));
                    }
                }
            });
        });
    }

    pub fn show_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |_| {
            let mut added_nodes = Vec::new();
            DockArea::new(&mut self.state.tree)
                .style({
                    let mut style = Style::from_egui(ctx.style().as_ref());
                    style.tab_bar.fill_tab_bar = true;
                    style
                })
                .show(
                    ctx,
                    &mut TabViewer {
                        added_nodes: &mut added_nodes,
                    },
                );
    
            added_nodes.drain(..).for_each(|(surface, node)| {
                self.state.tree.set_focused_node_and_surface((surface, node));
                let home_screen = HomeScreen::default();
                self.state.tree.push_to_focused_leaf(Utils::get_wrapped(home_screen));
                self.state.counter += 1;
            });
        });
    }

    fn keybindings(&mut self, ctx: &egui::Context) {
        #[cfg(not(target_arch = "wasm32"))]
        if ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::F11)) {
            let fullscreen = ctx.input(|i| i.viewport().fullscreen.unwrap_or(false));
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fullscreen));
        }
    }
}
