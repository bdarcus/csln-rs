// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Style;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Style = serde_json::from_str(&json).unwrap();
// }

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod options;
use options::OptionGroup;

mod template;
use template::*;

/// A CSL Style.
#[derive(Serialize, Deserialize)]
pub struct Style {
    /// The bibliography specification.
    pub bibliography: Option<BibliographyStyle>,
    /// The categories the style belongs to; for purposes of indexing.
    pub categories: Option<Vec<StyleCategory>>,
    /// The citation specification.
    pub citation: Option<CitationStyle>,
    /// The description of the style.
    pub description: Option<String>,
    /// The machine-readable token that uniquely identifies the style.
    pub id: Option<String>,
    pub options: Option<OptionGroup>,
    pub templates: Option<NamedTemplate>,
    /// The human-readable name of the style.
    pub title: Option<String>,
}

/// The bibliography specification.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BibliographyStyle {
    pub heading: Option<String>,
    pub list_style: Option<String>,
    pub options: Option<OptionGroup>,
    pub template: Option<Vec<TemplateComponent>>,
}
