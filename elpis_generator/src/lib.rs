mod sheet;

use std::error::Error as StdError;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::str;

use serde_json::Error as JsonError;
use thiserror::Error as ThisError;

use self::sheet::SheetDefinition;

pub type Result<T> = core::result::Result<T, Box<dyn StdError>>;

#[non_exhaustive]
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Missing definition files")]
    MissingDefinitions,
    #[error("Failed to parse file at: {path}\n{err}")]
    ParseError { path: PathBuf, err: JsonError },
}

#[derive(Clone, Debug)]
pub struct CodeGenerator {
    root: PathBuf,
    template: String,
    definitions: Vec<SheetDefinition>,
}

impl CodeGenerator {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        if !path.as_ref().exists() || !path.as_ref().is_dir() {
            Err(Box::new(Error::MissingDefinitions))
        } else {
            let template = ::std::str::from_utf8(include_bytes!("struct.rs.template"))?.to_string();
            let mut definitions = Vec::new();
            let mut iter = ::std::fs::read_dir(path.as_ref())?;
            while let Some(Ok(entry)) = iter.next() {
                let e_path = entry.path();
                if e_path.is_file() {
                    if let Some(extension) = e_path.extension().and_then(OsStr::to_str) {
                        if !extension.contains("json") {
                            continue;
                        }
                    } else {
                        continue;
                    }
                    let file = ::std::fs::read_to_string(&e_path)?;
                    let sheet = match serde_json::from_str::<SheetDefinition>(&file) {
                        Ok(sheet) => sheet,
                        Err(e) => {
                            return Err(Box::new(Error::ParseError {
                                path: e_path,
                                err: e,
                            }))
                        }
                    };
                    definitions.push(sheet);
                }
            }
            if definitions.is_empty() {
                Err(Box::new(Error::MissingDefinitions))
            } else {
                Ok(Self {
                    root: path.as_ref().to_path_buf(),
                    template,
                    definitions,
                })            
            }
        }
    }

    pub fn fetch_version(&self) -> Result<String> {
        let path = self.root.join("game.ver");
        Ok(::std::fs::read_to_string(path)?)
    }

    pub fn fetch_structs(&self) -> Vec<String> {
        let mut result = Vec::new();
        for sheet in &self.definitions {
            let mut root_definition = String::new();
            for definition in &sheet.definitions {
                match definition {
                    Some(definition) => {

                    },
                    None => {
                        result.push(
                            self.template
                                .clone()
                                .replace("%%SHEET_NAME%%", &sheet.name)
                        );
                    },
                }
            }
        }
        result
    }
}

pub fn clean_str<S: AsRef<str>>(s: S) -> String {
    if s.as_ref().is_empty() {
        return String::new();
    }
    let mut result = s
        .as_ref()
        .replace(
            ['<', '>', '{', '}', '(', ')', '/', '[', ']', ' ', '\'', '-'],
            "",
        )
        .replace("%", "Pct");
    if result.chars().next().unwrap().is_numeric() {
        let index_word = match result.as_bytes()[0] - '0' as u8 {
            0 => "zero",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            _ => panic!("invalid single digit value"),
        };
        result.replace_range(0..1, index_word);
    }
    result
}
