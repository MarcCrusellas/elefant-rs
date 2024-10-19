use super::screens::editor::EditorScreen;
use super::screens::home::HomeScreen;
use super::pane::Pane;
use super::screens::AnchorScreen;
use super::tree::TreeBehavior;


pub struct State {
    pub selected_item : AnchorScreen,
    pub side_panel_expanded: bool,
    pub tree: egui_tiles::Tree<Pane>,
    pub tree_behavior: TreeBehavior
}

impl Default for State {
    fn default() -> Self {
        Self {
            selected_item: AnchorScreen::Home,
            side_panel_expanded: true,
            tree: State::create_tree(),
            tree_behavior: TreeBehavior::default()
        }
    }
}

impl State {
    fn create_tree() -> egui_tiles::Tree<Pane> {
        let mut tiles = egui_tiles::Tiles::default();
    
        let mut tabs = vec![];
        tabs.push(tiles.insert_pane(Pane {
            p_type: AnchorScreen::Home,
            view: Box::new(HomeScreen {
                name: "Home 1".to_owned(),
            }),
        }));
        tabs.push(tiles.insert_pane(Pane {
            p_type: AnchorScreen::Editor,
            view: Box::new(EditorScreen {
                name: "Home 2".to_owned(),
            }),
        }));
    
        let root = tiles.insert_tab_tile(tabs);
    
        egui_tiles::Tree::new("my_tree", root, tiles)
    }
}


