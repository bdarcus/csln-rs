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

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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
    pub templates: Option<HashMap<String, Option<serde_json::Value>>>,
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

/// Parameter groups.
#[derive(Serialize, Deserialize)]
pub struct OptionGroup {
    /// Contributor list formatting configuration.
    pub contributors: Option<ContributorListFormatting>,
    /// Date formatting configuration.
    pub dates: Option<DateFormatting>,
    /// Disambiguation configuration of rendererd group display names.
    pub disambiguate: Option<Disambiguation>,
    /// Grouping configuration.
    pub group: Option<Vec<GroupSortKeys>>,
    /// Localization configuration.
    pub localization: Option<Localization>,
    /// Sorting configuration.
    pub sort: Option<Vec<SortConfig>>,
    /// Substitution configuration.
    pub substitute: Option<Substitution>,
}

/// Contributor list formatting configuration.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributorListFormatting {
    /// The delimiter between last and second-to-last item.
    ///
    /// The default "text" value produces:
    ///
    /// >  Doe, Johnson and Smith
    ///
    /// The "symbol" value produces:
    ///
    /// >  Doe, Johnson & Smith
    pub and_as: Option<AndAsString>,
    pub bold: Option<bool>,
    /// Format a contributor name as sorted.
    pub display_as_sort: Option<ContributorScope>,
    pub emph: Option<bool>,
    /// Configuring of the display of contributor rolee annotations.
    pub role: Option<RoleOption>,
    /// Configuration for contributor list shortening.
    pub shorten: Option<ContributorListShortening>,
    /// The symbol pair to wrap around one or more rendering components.
    /// Interaction with surrounding punctuation is localized.
    pub wrap: Option<WrapPunctuation>,
}

/// The delimiter between last and second-to-last item.
///
/// The default "text" value produces:
///
/// >  Doe, Johnson and Smith
///
/// The "symbol" value produces:
///
/// >  Doe, Johnson & Smith
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AndAsString {
    Symbol,
    Text,
}

/// Format a contributor name as sorted.
///
/// Which of the contributor names in a list to apply the transformation.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContributorScope {
    All,
    First,
    None,
}

/// Configuring of the display of contributor rolee annotations.
///
/// Contributor role configuration.
#[derive(Serialize, Deserialize)]
pub struct RoleOption {
    pub bold: Option<bool>,
    pub emph: Option<bool>,
    /// The display form of the role.
    ///
    /// ### `long`
    ///
    /// The full name of the role.
    ///
    /// > Jane Smith (editor)
    ///
    /// ### `short`
    ///
    /// > Jane Smith (ed.)
    ///
    /// ### `verb`
    ///
    /// > edited by Jane Smith
    ///
    /// ### `verb-short`
    ///
    /// > ed. Jane Smith
    pub form: Option<Form>,
    /// Contributor roles for which to omit the role description.
    ///
    /// The default value is `["author"]`, which omits the role for authors, including for any
    /// author substitutions.
    pub omit: Option<Vec<ContributorRoles>>,
    /// The symbol pair to wrap around one or more rendering components.
    /// Interaction with surrounding punctuation is localized.
    pub wrap: Option<WrapPunctuation>,
}

/// The display form of the role.
///
/// ### `long`
///
/// The full name of the role.
///
/// > Jane Smith (editor)
///
/// ### `short`
///
/// > Jane Smith (ed.)
///
/// ### `verb`
///
/// > edited by Jane Smith
///
/// ### `verb-short`
///
/// > ed. Jane Smith
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Form {
    Long,
    Short,
    Verb,
    #[serde(rename = "verb-short")]
    VerbShort,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContributorRoles {
    Author,
    Editor,
    Publisher,
}

/// The symbol pair to wrap around one or more rendering components.
/// Interaction with surrounding punctuation is localized.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WrapPunctuation {
    Brackets,
    Parentheses,
    Quotes,
}

/// Configuration for contributor list shortening.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributorListShortening {
    /// When shortening, replace omitted names with this term form.
    ///
    /// ### `short`
    ///
    /// > Doe, Johnson, et al.
    ///
    /// ### `long`
    ///
    /// > Doe, Johnson, and others
    pub at_al: Option<AtAl>,
    /// Determines when the delimiter or a space is used between a truncated contributor list
    /// and the “et-al” (or “and others”) term in case of et-al abbreviation.
    ///
    /// Allowed values:
    ///
    /// ### `contextual`
    ///
    /// Delimiter is only used for contributor lists truncated to two or more items:
    ///
    /// - 1 name: “J. Doe et al.”
    /// - 2 names: “J. Doe, S. Smith, et al.”
    ///
    /// ### `after-inverted-name`
    ///
    /// Delimiter is only used if the preceding name is inverted as a result of the `asSort`
    /// parameter.
    /// E.g. with `asSort` set to “first”:
    ///
    /// - “Doe, J., et al.”
    /// - “Doe, J., S. Smith et al.”
    ///
    /// ### `always`
    ///
    /// Delimiter is always used:
    ///
    /// - 1 name: “J. Doe, et al.”
    /// - 2 names: “J. Doe, S. Smith, et al.”
    ///
    /// ### `never`
    ///
    /// Delimiter is never used:
    /// - 1 name: “J. Doe et al.”
    /// - 2 names: “J. Doe, S. Smith et al.”
    pub delimiter_precedes_et_al: Option<DelimiterPrecedes>,
    /// Determines when the delimiter is used to separate the second to last and the last
    /// item in contributor lists (if `and` is not set, the name delimiter is always used,
    /// regardless of the value of `delimiterPrecedesLast`). Allowed values:
    ///
    /// ### `contextual`
    ///
    /// The contributor delimiter is only used for lists of three or more:
    ///
    /// - 2 names: “J. Doe and T. Williams”
    /// - 3 names: “J. Doe, S. Smith, and T. Williams”
    ///
    /// ### `after-inverted-name`
    ///
    /// Delimiter is only used if the preceding name is inverted as a result of the
    /// `asSort` parameter. E.g. with `asSort` set to “first”:
    ///
    /// - “Doe, J., and T. Williams”
    /// -  “Doe, J., S. Smith and T. Williams”
    ///
    /// ### `always`
    ///
    /// Delimiter is always used:
    ///
    /// - 2 names: “J. Doe, and T. Williams”
    /// - 3 names: “J. Doe, S. Smith, and T. Williams”
    ///
    /// ### `never`
    ///
    /// Delimiter is never used:
    ///
    /// - 2 names: “J. Doe and T. Williams”
    /// - 3 names: “J. Doe, S. Smith and T. Williams”
    pub delimiter_precedes_last: Option<DelimiterPrecedes>,
    /// The minimum length of the before acitvating shortening
    pub min: Option<f64>,
    /// When shortening, use the first number of contributors.
    pub use_first: Option<f64>,
}

/// When shortening, replace omitted names with this term form.
///
/// ### `short`
///
/// > Doe, Johnson, et al.
///
/// ### `long`
///
/// > Doe, Johnson, and others
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AtAl {
    Long,
    Short,
}

/// Determines when the delimiter or a space is used between a truncated contributor list
/// and the “et-al” (or “and others”) term in case of et-al abbreviation.
///
/// Allowed values:
///
/// ### `contextual`
///
/// Delimiter is only used for contributor lists truncated to two or more items:
///
/// - 1 name: “J. Doe et al.”
/// - 2 names: “J. Doe, S. Smith, et al.”
///
/// ### `after-inverted-name`
///
/// Delimiter is only used if the preceding name is inverted as a result of the `asSort`
/// parameter.
/// E.g. with `asSort` set to “first”:
///
/// - “Doe, J., et al.”
/// - “Doe, J., S. Smith et al.”
///
/// ### `always`
///
/// Delimiter is always used:
///
/// - 1 name: “J. Doe, et al.”
/// - 2 names: “J. Doe, S. Smith, et al.”
///
/// ### `never`
///
/// Delimiter is never used:
/// - 1 name: “J. Doe et al.”
/// - 2 names: “J. Doe, S. Smith et al.”
///
/// Determines when the delimiter is used to separate the second to last and the last
/// item in contributor lists (if `and` is not set, the name delimiter is always used,
/// regardless of the value of `delimiterPrecedesLast`). Allowed values:
///
/// ### `contextual`
///
/// The contributor delimiter is only used for lists of three or more:
///
/// - 2 names: “J. Doe and T. Williams”
/// - 3 names: “J. Doe, S. Smith, and T. Williams”
///
/// ### `after-inverted-name`
///
/// Delimiter is only used if the preceding name is inverted as a result of the
/// `asSort` parameter. E.g. with `asSort` set to “first”:
///
/// - “Doe, J., and T. Williams”
/// -  “Doe, J., S. Smith and T. Williams”
///
/// ### `always`
///
/// Delimiter is always used:
///
/// - 2 names: “J. Doe, and T. Williams”
/// - 3 names: “J. Doe, S. Smith, and T. Williams”
///
/// ### `never`
///
/// Delimiter is never used:
///
/// - 2 names: “J. Doe and T. Williams”
/// - 3 names: “J. Doe, S. Smith and T. Williams”
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DelimiterPrecedes {
    Always,
    Contextual,
    Never,
}

/// Date formatting configuration.
#[derive(Serialize, Deserialize)]
pub struct DateFormatting {
    pub date: Option<EStyle>,
    pub month: Option<MonthStyle>,
    pub time: Option<EStyle>,
    pub year: Option<YearStyle>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EStyle {
    Full,
    Long,
    Medium,
    Short,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MonthStyle {
    Long,
    Narrow,
    Numeric,
    Short,
    #[serde(rename = "2-digit")]
    The2Digit,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum YearStyle {
    Numeric,
    #[serde(rename = "2-digit")]
    The2Digit,
}

/// Disambiguation configuration of rendererd group display names.
///
/// Disambiguation of rendered group display name configuration.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Disambiguation {
    pub add_names: Option<AddNames>,
    pub add_year_suffix: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AddNames {
    All,
    #[serde(rename = "all-with-initials")]
    AllWithInitials,
    #[serde(rename = "by-cite")]
    ByCite,
    Primary,
    #[serde(rename = "primary-with-initials")]
    PrimaryWithInitials,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GroupSortKeys {
    #[serde(rename = "as-cited")]
    AsCited,
    Author,
    Title,
    Year,
}

/// Localization configuration.
///
/// Terms and data localization configuration.
#[derive(Serialize, Deserialize)]
pub struct Localization {
    /// The scope to use for localization.
    ///
    /// "per-item" uses the locale of the reference item, and "global" uses the target language
    /// across all references.
    pub scope: Option<Scope>,
}

/// The scope to use for localization.
///
/// "per-item" uses the locale of the reference item, and "global" uses the target language
/// across all references.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Scope {
    Global,
    #[serde(rename = "per-item")]
    PerItem,
}

/// Reference sorting configuration.
#[derive(Serialize, Deserialize)]
pub struct SortConfig {
    pub key: Option<GroupSortKeys>,
    pub order: Option<Order>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Order {
    Ascending,
    Descending,
}

/// Substitution configuration.
///
/// Substitution of variable configuration.
#[derive(Serialize, Deserialize)]
pub struct Substitution {
    /// When author is nil, substitute the first non-nil listed variable.
    /// Once a substitution is made, the substituted variable shall be set to nil for purposes of
    /// later rendering.
    pub author: Option<Vec<Substitute>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Substitute {
    Editor,
    Title,
    Translator,
}

/// A template defined inline.
///
/// A template is called by name.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    /// The conditions that must be true for the templates to render.
    #[serde(rename = "match")]
    pub condition_match: Option<Match>,
    pub template: Option<Vec<TemplateComponent>>,
    /// Is the item variable a number?
    pub is_number: Option<Locators>,
    /// Does the date conform to EDTF?
    #[serde(rename = "isEDTFDate")]
    pub is_edtf_date: Option<Dates>,
    /// Is the item reference type among the listed reference types?
    pub is_ref_type: Option<Vec<ReferenceTypes>>,
    /// Does the item reference include one of the listed variables?
    pub has_variable: Option<Vec<Variables>>,
    /// The item reference locale; to allow multilingual output.
    pub locale: Option<String>,
    pub template_key: Option<String>,
}

/// When all of the when conditions are nil, format the children.
///
/// The rendering of style templates can be specified by reference to a template name or by
/// inline definition.
///
/// A template defined inline.
///
/// A template is called by name.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    pub template: Option<Vec<TemplateComponent>>,
    pub template_key: Option<String>,
}

/// A component of a CSL style template.
///
/// Integral citations are those where the author is printed inline in the text; aka "in
/// text" or "narrative" citations.
///
/// Non-integral citations are those where the author is incorporated in the citation, and
/// not printed inline in the text.
///
/// A template is called by name.
///
/// A template component for rendering contributors.
///
/// A template component for rendering locators.
///
/// A template component for rendering dates.
///
/// A template component for rendering title.
///
/// Non-localized plain text.
///
/// Localized strings.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateComponent {
    pub options: Option<OptionGroup>,
    pub template: Option<Vec<TemplateComponent>>,
    pub template_key: Option<String>,
    pub bold: Option<bool>,
    pub emph: Option<bool>,
    pub variable: Option<SimpleTypes>,
    /// The symbol pair to wrap around one or more rendering components.
    /// Interaction with surrounding punctuation is localized.
    pub wrap: Option<WrapPunctuation>,
    pub contributor: Option<ContributorRoles>,
    pub locator: Option<Locators>,
    pub date: Option<Dates>,
    pub format: Option<Format>,
    pub title: Option<Titles>,
    pub text: Option<String>,
    pub term: Option<LocalizedTermName>,
    /// When all of the when conditions are nil, format the children.
    #[serde(rename = "else")]
    pub template_component_else: Option<Template>,
    /// For the first condition that is non-nil, format the children.
    pub when: Option<Vec<Condition>>,
}

/// The conditions that must be true for the templates to render.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Match {
    All,
    Any,
    None,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Variables {
    Accessed,
    Article,
    Author,
    Book,
    Chapter,
    #[serde(rename = "container-title")]
    ContainerTitle,
    Editor,
    Issue,
    Issued,
    Pages,
    Publisher,
    Title,
    Volume,
}

/// Does the date conform to EDTF?
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Dates {
    Accessed,
    Issued,
}

/// Is the item variable a number?
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Locators {
    Chapter,
    Page,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReferenceTypes {
    Article,
    Book,
    Chapter,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Format {
    Full,
    Main,
    #[serde(rename = "month-day")]
    MonthDay,
    Short,
    Sub,
    Symbol,
    Year,
    #[serde(rename = "year-month")]
    YearMonth,
}

/// The unique human-readable identifier for a term.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LocalizedTermName {
    Accessed,
    Act,
    Ad,
    #[serde(rename = "advance-online-publication")]
    AdvanceOnlinePublication,
    Album,
    And,
    #[serde(rename = "and-others")]
    AndOthers,
    Anonymous,
    Appendix,
    #[serde(rename = "article-locator")]
    ArticleLocator,
    At,
    #[serde(rename = "audio-recording")]
    AudioRecording,
    #[serde(rename = "available-at")]
    AvailableAt,
    Bc,
    Bce,
    Book,
    By,
    Canon,
    Ce,
    Chapter,
    Circa,
    Cited,
    Column,
    Elocation,
    Equation,
    #[serde(rename = "et-al")]
    EtAl,
    Figure,
    Film,
    Folio,
    Forthcoming,
    From,
    Henceforth,
    Ibid,
    In,
    #[serde(rename = "in-press")]
    InPress,
    Internet,
    Interview,
    Issue,
    Letter,
    Line,
    #[serde(rename = "loc-cit")]
    LocCit,
    #[serde(rename = "no date")]
    NoDate,
    #[serde(rename = "no-place")]
    NoPlace,
    #[serde(rename = "no-publisher")]
    NoPublisher,
    Note,
    On,
    Online,
    #[serde(rename = "op-cit")]
    OpCit,
    Opus,
    #[serde(rename = "original-work-published")]
    OriginalWorkPublished,
    Page,
    Paragraph,
    Part,
    #[serde(rename = "personal-communication")]
    PersonalCommunication,
    Podcast,
    #[serde(rename = "podcast-episode")]
    PodcastEpisode,
    Preprint,
    #[serde(rename = "presented-at")]
    PresentedAt,
    #[serde(rename = "radio-broadcast")]
    RadioBroadcast,
    #[serde(rename = "radio-series")]
    RadioSeries,
    #[serde(rename = "radio-series-episode")]
    RadioSeriesEpisode,
    Reference,
    Retrieved,
    #[serde(rename = "review-of")]
    ReviewOf,
    Rule,
    Scale,
    Scene,
    Section,
    #[serde(rename = "special-issue")]
    SpecialIssue,
    #[serde(rename = "special-section")]
    SpecialSection,
    #[serde(rename = "sub-verbo")]
    SubVerbo,
    Supplement,
    Table,
    #[serde(rename = "television-broadcast")]
    TelevisionBroadcast,
    #[serde(rename = "television-series")]
    TelevisionSeries,
    #[serde(rename = "television-series-episode")]
    TelevisionSeriesEpisode,
    Timestamp,
    #[serde(rename = "title-locator")]
    TitleLocator,
    Verse,
    Version,
    Video,
    Volume,
    #[serde(rename = "working-paper")]
    WorkingPaper,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Titles {
    #[serde(rename = "container-title")]
    ContainerTitle,
    Title,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SimpleTypes {
    Issue,
    Pages,
    Volume,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StyleCategory {
    Biology,
    Science,
    #[serde(rename = "social science")]
    SocialScience,
}

/// The citation specification.
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationStyle {
    /// Integral citations are those where the author is printed inline in the text; aka "in
    /// text" or "narrative" citations.
    pub integral: Option<RenderList>,
    /// Non-integral citations are those where the author is incorporated in the citation, and
    /// not printed inline in the text.
    pub non_integral: Option<RenderList>,
    pub options: Option<OptionGroup>,
    pub placement: Option<Placement>,
    pub template: Option<Vec<TemplateComponent>>,
}

/// Integral citations are those where the author is printed inline in the text; aka "in
/// text" or "narrative" citations.
///
/// Non-integral citations are those where the author is incorporated in the citation, and
/// not printed inline in the text.
#[derive(Serialize, Deserialize)]
pub struct RenderList {
    pub options: Option<OptionGroup>,
    pub template: Option<Vec<TemplateComponent>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Placement {
    Inline,
    Note,
}
