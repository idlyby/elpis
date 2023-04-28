use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::game::Language;

pub mod prelude {
    pub use super::Game;
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Game {
    pub path: PathBuf,
    pub config_path: PathBuf,
    pub is_dx11: bool,
    pub is_steam: bool,
    pub is_free_trial: bool,
    pub language: Language,
    pub encrypt_args: bool,
    pub extra_args: Option<String>,
    pub account_id: Option<String>,
}

impl Default for Game {
    fn default() -> Self {
        let launcher_dir = crate::consts::launcher_dir();
        Self {
            path: launcher_dir.join("ffxiv"),
            config_path: launcher_dir.join("ffxiv_config"),
            is_dx11: true,
            is_steam: false,
            is_free_trial: false,
            language: Language::default(),
            encrypt_args: true,
            extra_args: None,
            account_id: None,
        }
    }
}

impl From<&Value> for Game {
    fn from(value: &Value) -> Self {
        let default = Self::default();
        Self {
            path: match value.get("path") {
                Some(value) => match value.as_str() {
                    Some(value) => PathBuf::from(value),
                    None => default.path,
                },
                None => default.path,
            },
            config_path: match value.get("config_path") {
                Some(value) => match value.as_str() {
                    Some(value) => PathBuf::from(value),
                    None => default.config_path,
                },
                None => default.config_path,
            },
            is_dx11: match value.get("is_dx11") {
                Some(value) => match value.as_bool() {
                    Some(value) => value,
                    None => default.is_dx11,
                },
                None => default.is_dx11,
            },
            is_steam: match value.get("is_steam") {
                Some(value) => match value.as_bool() {
                    Some(value) => value,
                    None => default.is_steam,
                },
                None => default.is_steam,
            },
            is_free_trial: match value.get("is_free_trial") {
                Some(value) => match value.as_bool() {
                    Some(value) => value,
                    None => default.is_free_trial,
                },
                None => default.is_free_trial,
            },
            language: match value.get("language") {
                Some(value) => Language::from(value),
                None => default.language,
            },
            encrypt_args: match value.get("encrypt_args") {
                Some(value) => match value.as_bool() {
                    Some(value) => value,
                    None => default.encrypt_args,
                },
                None => default.encrypt_args,
            },
            extra_args: match value.get("extra_args") {
                Some(value) => {
                    if value.is_null() {
                        None
                    } else {
                        match value.as_str() {
                            Some(value) => Some(value.to_string()),
                            None => default.extra_args,
                        }
                    }
                },
                None => default.extra_args,
            },
            account_id: match value.get("account_id") {
                Some(value) => {
                    if value.is_null() {
                        None
                    } else {
                        match value.as_str() {
                            Some(value) => Some(value.to_string()),
                            None => default.account_id,
                        }
                    }
                },
                None => default.account_id,
            },
        }
    }
}