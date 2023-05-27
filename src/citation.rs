// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Citation;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Citation = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Citation {
    /// Local citation rendering option; aka command or style.
    ///
    /// - `integral` places the author inline in the text; also known as "narrative" or "in text"
    /// citations.
    /// - `nonIntegral` places the author in the citation.
    ///
    /// Both are more general than author-date styles, and can apply to any citation style.
    pub mode: Option<CitationModeType>,
    /// The string that prefaces a list of citation references.
    pub prefix: Option<String>,
    pub references: Option<Vec<Reference>>,
    /// A string that follows a list of citation references.
    pub suffix: Option<String>,
}

/// Local citation rendering option; aka command or style.
///
/// - `integral` places the author inline in the text; also known as "narrative" or "in text"
/// citations.
/// - `nonIntegral` places the author in the citation.
///
/// Both are more general than author-date styles, and can apply to any citation style.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CitationModeType {
    Integral,
    #[serde(rename = "nonIntegral")]
    NonIntegral,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reference {
    /// A string that prefaces the citation reference.
    pub prefix: Option<String>,
    /// The unique identifier token for the citation reference.
    #[serde(rename = "refID")]
    pub ref_id: Option<String>,
    /// An array of locator key-values and/or strings.
    pub suffix: Option<Vec<Locator>>,
}

/// A key-value object, or a string.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Locator {
    RecordLocatorTermsString(RecordLocatorTermsString),
    String(String),
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RecordLocatorTermsString {
    pub book: Option<String>,
    pub chapter: Option<String>,
    pub column: Option<String>,
    pub figure: Option<String>,
    pub folio: Option<String>,
    pub line: Option<String>,
    pub note: Option<String>,
    pub number: Option<String>,
    pub opus: Option<String>,
    pub page: Option<String>,
    pub paragraph: Option<String>,
    pub part: Option<String>,
    pub section: Option<String>,
    pub sub_verbo: Option<String>,
    pub verse: Option<String>,
    pub volume: Option<String>,
}
