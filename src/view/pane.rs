use super::screens::{communication::PaneComponent, AnchorScreen};

pub struct Pane {
    pub p_type: AnchorScreen,
    pub view: Box<dyn PaneComponent>
}

