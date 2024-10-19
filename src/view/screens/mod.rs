pub mod editor;

pub mod home;

pub mod communication;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum AnchorScreen {
    #[default]
    Undefined,
    Home,
    Editor
}

