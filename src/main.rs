#![windows_subsystem = "windows"]

use eframe::egui;
use view::main_window::MainEditor;

mod model;
mod utils;
mod view;

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


// // fn ui_example_system(mut contexts: EguiContexts) {
// //     // MainEditor::default().show(&contexts.context());


// // }


// fn configure_visuals(mut egui_ctx: bevy_egui::EguiContexts) {
//     egui_ctx.ctx_mut().set_visuals(egui::Visuals {
//         window_rounding: 0.0.into(),
//         ..Default::default()
//     });
// }

// fn configure_ui_state(mut ui_state: ResMut<MainEditor>) {
//     ui_state.state.is_window_open = true;
// }


// fn ui_example(
//     mut egui_ctx: EguiContexts,
//     mut ui_state: ResMut<MainEditor>,
//     mut rendered_texture_id: Local<Option<egui::TextureId>>,
// ) {

//     ui_state.update(egui_ctx.ctx_mut(), frame);
// }