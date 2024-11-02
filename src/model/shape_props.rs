

use serde::{Deserialize, Deserializer, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Selrect {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
    pub rotation: f32,
}

impl Default for Selrect {
    fn default() -> Self {
        Selrect {
            x: 0,
            y: 0,
            width: 10,
            height: 10,
            rotation: 0.0,
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
