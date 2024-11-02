#[cfg(test)]
mod tests {
    use crate::model::{events::*, shape::*, shape_props::*};

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
            event: Events::default(),
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
                rotation: 90.0,
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
            event: Events::default()
                .click("bla bla")
                .hover("ddd"),
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
                rotation: 0.0,
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
            event: Events::default(),
        };

        let json = serde_json::to_string(&shape).unwrap();
        let deserialized_shape: ShapeModel = serde_json::from_str(&json).unwrap();
        assert_eq!(shape, deserialized_shape);
    }

    #[test]
    fn test_event_prototype_default() {
        let _context = setup();
        let event = EventPrototype {
            is_available: false,
            event_script: None,
        };

        let json = serde_json::to_string(&event).unwrap();
        let deserialized_event: EventPrototype = serde_json::from_str(&json).unwrap();
        assert_eq!(event, deserialized_event);
    }
}
