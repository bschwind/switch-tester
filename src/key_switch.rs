use dioxus::html::input_data::keyboard_types::Code;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeySwitch {
    pub name: String,
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SwitchData {
    pub switches: HashMap<String, KeySwitch>,
    pub map: HashMap<Code, String>,
}
