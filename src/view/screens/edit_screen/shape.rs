//! ```
//!  Shapes#
//! ShapeModel
//! A Shape is the most important entity of the model. Represents one of the layers of our design, and it corresponds with one SVG node, augmented with Penpot special features.

//! We have code to render a Shape into a SVG tag, with more or less additions depending on the environment (editable in the workspace, interactive in the viewer, minimal in the shape exporter or the handoff, or with metadata in the file export).

//! Also have code that imports any SVG file and convert elements back into shapes. If it's a SVG exported by Penpot, it reads the metadata to reconstruct the shapes exactly as they were. If not, it infers the atributes with a best effort approach.

//! In addition to the identifier ones (the id, the name and the type of element), a shape has a lot of attributes. We tend to group them in related clusters. Those are the main ones:

//! Selrect and other geometric attributes (x, y, width, height...) define the position in the diagram and the bounding box.
//! Transform is a 2D transformation matrix to rotate or stretch the shape.
//! Constraints explains how the shape changes when the container shape resizes (kind of "responsive" behavior).
//! Interactions describe the interactive behavior when the shape is displayed in the viewer.
//! Fill contains the shape fill color and options.
//! Stroke contains the shape stroke color and options.
//! Shadow contains the shape shadow options.
//! Blur contains the shape blur options.
//! Font contains the font options for a shape of type text.
//! Content contains the text blocks for a shape of type text.
//! Exports are the defined export settings for the shape.
//! Event contains the event settings for the shape.
//! ```
//!
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Selrect {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
}

impl Default for Selrect {
    fn default() -> Self {
        Selrect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transform {
    pub options: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Constraints {
    pub options: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Interactions {
    pub options: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Fill {
    pub color: String,
    pub options: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Stroke {
    pub color: String,
    pub options: String,
}

fn deserialize_f64_null_as_nan<'de, D: Deserializer<'de>>(des: D) -> Result<f32, D::Error> {
    let optional = Option::<f64>::deserialize(des)?;
    Ok(optional.unwrap_or(f64::NAN) as f32)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Vec2 {
    /// Rightwards. Width.
    #[serde(deserialize_with = "deserialize_f64_null_as_nan")]
    pub x: f32,

    /// Downwards. Height.
    #[serde(deserialize_with = "deserialize_f64_null_as_nan")]
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Shadow {
    pub offset: Vec2,
    pub blur: f32,
    pub spread: f32,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Blur {
    pub options: String,
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Font {
    pub options: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TextBlock {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Content {
    pub text_blocks: Vec<TextBlock>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Exports {
    pub settings: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct Event {
    pub click: Option<String>,
    pub hover: Option<String>,
    pub double_click: Option<String>,
    pub long_press: Option<String>,
    pub when_visible: Option<String>,
    pub when_hidden: Option<String>,
    pub when_focused: Option<String>,
    pub when_unfocused: Option<String>,
    pub when_apears: Option<String>,
}

// all attributes of a shape are optional
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ShapeModel {
    pub id: i16,
    pub name: String,
    pub shape_type: String,
    pub selrect: Selrect,
    pub transform: Option<Transform>,
    pub constraints: Option<Constraints>,
    pub interactions: Option<Interactions>,
    pub fill: Option<Fill>,
    pub stroke: Option<Stroke>,
    pub shadow: Option<Shadow>,
    pub blur: Option<Blur>,
    pub font: Option<Font>,
    pub content: Option<Content>,
    pub exports: Option<Exports>,
    pub event: Event,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    struct TestContext {
        // Add any fields that need to be cleaned up after each test
    }

    impl Drop for TestContext {
        fn drop(&mut self) {
            // Clean up code here
        }
    }

    fn setup() -> TestContext {
        // Setup code here
        TestContext {
                // Initialize fields
            }
    }

    #[test]
    fn test_shape_model() {
        let _context = setup();
        let shape = ShapeModel {
            id: 1,
            name: "shape1".to_string(),
            shape_type: "rect".to_string(),
            selrect: Selrect::default(),
            transform: Some(Transform {
                options: "rotate(45)".to_string(),
            }),
            constraints: Some(Constraints {
                options: "center".to_string(),
            }),
            interactions: Some(Interactions {
                options: "hover".to_string(),
            }),
            fill: Some(Fill {
                color: "red".to_string(),
                options: "solid".to_string(),
            }),
            stroke: Some(Stroke {
                color: "black".to_string(),
                options: "solid".to_string(),
            }),
            shadow: Some(Shadow {
                offset: Vec2 { x: 1.0, y: 1.0 },
                blur: 1.0,
                spread: 1.0,
                color: "black".to_string(),
            }),
            blur: Some(Blur {
                options: "solid".to_string(),
            }),
            font: Some(Font {
                options: "Arial".to_string(),
            }),
            content: Some(Content {
                text_blocks: vec![TextBlock {
                    text: "Hello".to_string(),
                }],
            }),
            exports: Some(Exports {
                settings: "pdf".to_string(),
            }),
            event: Event::default(),
        };

        let json = serde_json::to_string(&shape).unwrap();
        let deserialized_shape: ShapeModel = serde_json::from_str(&json).unwrap();
        assert_eq!(shape, deserialized_shape);
    }

    #[test]
    fn test_shape_model_with_different_values() {
        let _context = setup();
        let shape = ShapeModel {
            id: 2,
            name: "shape2".to_string(),
            shape_type: "circle".to_string(),
            selrect: Selrect {
                x: 5,
                y: 5,
                width: 20,
                height: 20,
            },
            transform: Some(Transform {
                options: "scale(2)".to_string(),
            }),
            constraints: Some(Constraints {
                options: "flex".to_string(),
            }),
            interactions: Some(Interactions {
                options: "click".to_string(),
            }),
            fill: Some(Fill {
                color: "blue".to_string(),
                options: "gradient".to_string(),
            }),
            stroke: Some(Stroke {
                color: "green".to_string(),
                options: "dashed".to_string(),
            }),
            shadow: Some(Shadow {
                offset: Vec2 { x: 2.0, y: 2.0 },
                blur: 2.0,
                spread: 2.0,
                color: "gray".to_string(),
            }),
            blur: Some(Blur {
                options: "light".to_string(),
            }),
            font: Some(Font {
                options: "Verdana".to_string(),
            }),
            content: Some(Content {
                text_blocks: vec![TextBlock {
                    text: "World".to_string(),
                }],
            }),
            exports: Some(Exports {
                settings: "svg".to_string(),
            }),
            event: Event {
                click: Some("click_event".to_string()),
                hover: Some("hover_event".to_string()),
                double_click: None,
                long_press: None,
                when_visible: None,
                when_hidden: None,
                when_focused: None,
                when_unfocused: None,
                when_apears: None,
            },
        };

        let json = serde_json::to_string(&shape).unwrap();
        let deserialized_shape: ShapeModel = serde_json::from_str(&json).unwrap();
        assert_eq!(shape, deserialized_shape);
    }

    #[test]
    fn test_shape_model_with_minimal_values() {
        let _context = setup();
        let shape = ShapeModel {
            id: 3,
            name: "shape3".to_string(),
            shape_type: "line".to_string(),
            selrect: Selrect {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
            },
            transform: None,
            constraints: None,
            interactions: None,
            fill: None,
            stroke: None,
            shadow: None,
            blur: None,
            font: None,
            content: None,
            exports: None,
            event: Event::default(),
        };

        let json = serde_json::to_string(&shape).unwrap();
        let deserialized_shape: ShapeModel = serde_json::from_str(&json).unwrap();
        assert_eq!(shape, deserialized_shape);
    }
}
