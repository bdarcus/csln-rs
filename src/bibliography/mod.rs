use serde::{Serialize, Deserialize};
use std::collections::HashMap;

mod reference;
use reference::InputReference;

pub type InputBibliography = HashMap<String, InputReference>;
