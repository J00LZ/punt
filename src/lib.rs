use std::collections::BTreeMap;
use std::env::VarError;

use serde::Deserialize;
use thiserror::Error;

pub type Result = std::result::Result<(), Error>;


#[derive(Debug, Error)]
pub enum Error {
    #[error("Error executing program: {0}")]
    Run(i32),
    #[error("Error creating command: {0}")]
    CommandCreate(std::io::Error),
    #[error("Error creating symlink: {0}")]
    Link(#[from] std::io::Error),
    #[error("Error converting to str")]
    Convert,
    #[error("Error expanding path: {0}")]
    Expand(#[from] shellexpand::LookupError<VarError>),
    #[error("Error parsing uname")]
    Uname,
}


#[derive(PartialOrd, PartialEq, Deserialize, Debug)]
pub struct Config {
    #[serde(default)]
    pub general: GeneralSettings,
    #[serde(default)]
    pub files: BTreeMap<String, Entry>,
}

#[derive(PartialOrd, PartialEq, Deserialize, Debug)]
#[serde(default)]
pub struct GeneralSettings {
    pub verbose: bool,
    pub default_tags: Vec<String>,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            verbose: false,
            default_tags: vec![],
        }
    }
}

#[derive(PartialOrd, PartialEq, Deserialize, Debug)]
pub struct Entry {
    #[serde(flatten)]
    pub ft: FileType,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(PartialOrd, PartialEq, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FileType {
    Link { dest: String },
    Exec,
}
