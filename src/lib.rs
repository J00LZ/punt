use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(PartialOrd, PartialEq, Deserialize, Debug)]
pub struct Config {
    pub general: BTreeMap<String, String>,
    pub files: BTreeMap<String, FileType>,
}

#[derive(PartialOrd, PartialEq, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FileType {
    Link { dest: String },
    Exec,
}
