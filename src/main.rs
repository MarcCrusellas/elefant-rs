#![windows_subsystem = "windows"]
use std::time::Duration;

use eframe::egui::{Color32, Shadow};
#[warn(unused_variables)]
#[warn(dead_code)]
use eframe::egui::{self, Image};
mod view;
use egui_notify::{Toast, Toasts};
use view::items::side_panel_item::SidePanelItem;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        ..Default::default()
    };

    eframe::run_native(
        "Editor",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MainEditor>::default())
        }),
    )
}

struct MainEditor {
    text: String,
    state: view::vm::State,
    side_panel_items: Vec<SidePanelItem<'static>>,
}

impl Default for MainEditor {
    fn default() -> Self {
        Self {
            text: "Hello World!".to_owned(),
            side_panel_items: vec![
                SidePanelItem {
                    id: view::screens::AnchorScreen::Home,
                    name: "Home".to_owned(),
                    selected: false,
                    icon: Image::from_uri("file://../../../assets/home.png"),
                },
                SidePanelItem {
                    id: view::screens::AnchorScreen::Editor,
                    name: "Editor".to_owned(),
                    selected: false,
                    icon: Image::from_uri(
                        "https://cdn-icons-png.flaticon.com/512/5273/5273716.png",
                    ),
                },
            ],
            state: view::vm::State::default(),
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
}

impl MainEditor {
    // fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
    //     let selected_anchor = self.state.selected_anchor;
    //     for (_name, anchor, app) in self.apps_iter_mut() {
    //         if anchor == selected_anchor || ctx.memory(|mem| mem.everything_is_visible()) {
    //             app.update(ctx, frame);
    //         }
    //     }
    // }

    fn show_top_panel(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Top Panel");
            });
        });
    }

    fn show_side_panel(&mut self, ctx: &egui::Context) {
        let collapsed_side_panel = egui::SidePanel::left("side_panel_collapsed")
            .default_width(70.0)
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
                    let len = self.side_panel_items.len();
                    for (i, item) in self.side_panel_items.iter_mut().enumerate() {
                        if item
                            .show(
                                ui,
                                self.state.side_panel_expanded,
                                self.state.selected_item == item.id,
                            )
                            .clicked()
                        {
                            self.state.selected_item = item.id;
                            Toast::basic("Message").duration(Some(Duration::from_secs(2))).show_progress_bar(true);

                            Toasts::default().with_shadow(Shadow {
                                offset: Default::default(),
                                blur: 30.0,
                                spread: 5.0,
                                color: Color32::from_black_alpha(70),
                            });
                            
                            // add a item to the tree
                            self.state.tree.tiles.insert_pane(view::pane::Pane {
                                p_type: item.id,
                                view: match item.id {
                                    view::screens::AnchorScreen::Home => {
                                        Box::new(view::screens::home::HomeScreen {
                                            name: "Home 1".to_owned(),
                                        })
                                    }
                                    view::screens::AnchorScreen::Editor => {
                                        Box::new(view::screens::editor::EditorScreen {
                                            name: "editor ".to_owned(),
                                        })
                                    }
                                    _ => Box::new(view::screens::home::HomeScreen {
                                        name: "else".to_owned(),
                                    }),
                                },
                            });
                        };
                        if i != len - 1 {
                            ui.separator();
                        }
                    }
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::BOTTOM), |ui| {
                    // ui.allocate_space(ui.available_size()); // Make the side panel expand to max space
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
        egui::CentralPanel::default().show(ctx, |ui| {
            self.state.tree.ui(&mut self.state.tree_behavior, ui);
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
