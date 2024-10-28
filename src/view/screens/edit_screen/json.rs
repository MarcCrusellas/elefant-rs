use serde::{Deserialize, Serialize};

/// view types
/// base view
/// - Position
///    - Height
///    - Width
///    - X
///    - Y
///    - Z?
///    - Rotation?
/// - Appearance
///    - BackgroundColor
///    - BorderColor
///    - BorderWidth
///    - BorderRadius
///    - BoxShadow: InnerShadow, OuterShadow
///    - Visibility: Hidden, Visible, Exact(int16)
/// 
/// Component
///   - Name
///   - Position
///   - Appearance
/// 
/// Rectangle : Component
/// 
/// 
/// Document : Component
///    - Title
///    - Description
///    - Position
///    - Appearance
///    - Children: Vec<Component>

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
enum Visibility {
    Hidden,
    Visible,
    Exact(i16),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
enum  BoxShadow {
    InnerShadow,
    OuterShadow,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct Position {
    height: i16,
    width: i16,
    x: i16,
    y: i16,
    z: i16,
    rotation: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct Appearance {
    background_color: String,
    border_color: String,
    border_width: i16,
    border_radius: i16,
    box_shadow: BoxShadow,
    visibility: Visibility,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct Component {
    name: String,
    position: Position,
    appearance: Appearance,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct Rectangle {
    component: Component,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
struct Document {
    title: String,
    description: String,
    position: Position,
    appearance: Appearance,
    children: Vec<Component>,
}
