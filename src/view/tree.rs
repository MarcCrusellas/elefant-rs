use std::sync::{Arc, Mutex};
use eframe::egui;
use egui_dock::{NodeIndex, SurfaceIndex};
use super::screens::PaneComponent;

pub struct TabViewer<'a> {
    pub added_nodes: &'a mut Vec<(SurfaceIndex, NodeIndex)>,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = Arc<Mutex<dyn PaneComponent>>;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.lock().unwrap().tab_title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.lock().unwrap().ui(ui);
    }

    fn on_add(&mut self, surface: SurfaceIndex, node: NodeIndex) {
        self.added_nodes.push((surface, node));
    }
    
    fn context_menu(
        &mut self,
        _ui: &mut egui::Ui,
        _tab: &mut Self::Tab,
        _surface: SurfaceIndex,
        _node: NodeIndex,
    ) {
    }
    
    fn id(&mut self, tab: &mut Self::Tab) -> egui::Id {
        egui::Id::new(self.title(tab).text())
    }
    
    fn on_tab_button(&mut self, _tab: &mut Self::Tab, _response: &egui::Response) {}
    
    fn closeable(&mut self, _tab: &mut Self::Tab) -> bool {
        true
    }
    
    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        tab.lock().unwrap().close()
    }
    
    fn add_popup(&mut self, _ui: &mut egui::Ui, _surface: SurfaceIndex, _node: NodeIndex) {}
    
    fn force_close(&mut self, _tab: &mut Self::Tab) -> bool {
        false
    }
    
    fn tab_style_override(&self, _tab: &Self::Tab, _global_style: &egui_dock::TabStyle) -> Option<egui_dock::TabStyle> {
        None
    }
    
    fn allowed_in_windows(&self, _tab: &mut Self::Tab) -> bool {
        true
    }
    
    fn clear_background(&self, _tab: &Self::Tab) -> bool {
        true
    }
    
    fn scroll_bars(&self, _tab: &Self::Tab) -> [bool; 2] {
        [true, true]
    }
}