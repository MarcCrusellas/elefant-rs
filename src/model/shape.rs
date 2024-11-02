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
use std::default;

use serde::{Deserialize, Deserializer, Serialize};

use super::{events::Events, shape_props::*};

// all attributes of a shape are optional
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ShapeModel {
    pub id: i16,
    pub name: String,
    pub shape_type: String,
    pub selrect: Selrect,
    // can contain child shapes, but don't know how to represent it yet
    // pub children: Option<Vec<ShapeModel>>,
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
    pub event: Events,
}

