use super::generic_analyzer;
use super::slide::SlideYml;
use crate::analyzer::generic_analyzer::YmlAnalyzer;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PresYml {
    pub pres: PresYmlData,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct PresYmlData {
    pub theme: Option<String>,
    pub slides: Vec<SlideYml>,
}

impl PresYml {
    pub fn from_file(filename: String) -> Result<Box<PresYml>, generic_analyzer::AnalysisErr> {
        let mut content = String::new();
        let file = File::open(filename);
        match file {
            Ok(mut file) => {
                file.read_to_string(&mut content);
                PresYml::new(content.as_str())
            }
            Err(err) => Err(generic_analyzer::AnalysisErr::OsErr(format!(
                "OsError: {}",
                err
            ))),
        }
    }
}

impl generic_analyzer::YmlAnalyzer for PresYml {}
