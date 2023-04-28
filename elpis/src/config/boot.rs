use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod prelude {
    pub use super::Boot;
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Boot {
    pub is_autologin: bool,
    pub do_version_check: bool,

    // unix opts, completely ignored on win
    pub use_custom_wine: Option<bool>,
    pub wine_binary_path: Option<PathBuf>,
    pub wine_debug_vars: Option<String>,
    pub use_gamemode: Option<bool>,
    pub use_dxvk_async: Option<bool>,
    pub use_esync: Option<bool>,
    pub use_fsync: Option<bool>,
    pub use_futex2: Option<bool>,
}

impl Default for Boot {
    fn default() -> Self {
        let launcher_dir = crate::consts::launcher_dir();
        if cfg!(windows) {
            Self {
                is_autologin: false,
                do_version_check: true,

                use_custom_wine: None,
                wine_binary_path: None,
                wine_debug_vars: None,
                use_gamemode: None,
                use_dxvk_async: None,
                use_esync: None,
                use_fsync: None,
                use_futex2: None,
            }
        } else {
            Self {
                is_autologin: false,
                do_version_check: true,

                use_custom_wine: Some(false),
                wine_binary_path: Some(PathBuf::from("/usr/bin")),
                wine_debug_vars: Some(String::from("-all")),
                use_gamemode: Some(false),
                use_dxvk_async: Some(true),
                use_esync: Some(true),
                use_fsync: Some(false),
                use_futex2: Some(false),
            }
        }
    }
}

impl From<&Value> for Boot {
    fn from(value: &Value) -> Self {
        let default = Self::default();
        Self {
            is_autologin: match value.get("is_autologin") {
                Some(value) => match value.as_bool() {
                    Some(value) => value,
                    None => default.is_autologin,
                },
                None => default.is_autologin,
            },
            do_version_check: match value.get("do_version_check") {
                Some(value) => match value.as_bool() {
                    Some(value) => value,
                    None => default.do_version_check,
                },
                None => default.do_version_check,
            },

            use_custom_wine: if cfg!(windows) {
                None
            } else {
                match value.get("use_custom_wine") {
                    Some(value) => match value.as_bool() {
                        Some(value) => Some(value),
                        None => default.use_custom_wine,
                    },
                    None => default.use_custom_wine,
                }
            },
            wine_binary_path: if cfg!(windows) {
                None
            } else {
                match value.get("wine_binary_path") {
                    Some(value) => match value.as_str() {
                        Some(value) => Some(PathBuf::from(value)),
                        None => default.wine_binary_path,
                    },
                    None => default.wine_binary_path,
                }
            },
            wine_debug_vars:  if cfg!(windows) {
                None
            } else {
                match value.get("wine_debug_vars") {
                    Some(value) => match value.as_str() {
                        Some(value) => Some(value.to_string()),
                        None => default.wine_debug_vars,
                    },
                    None => default.wine_debug_vars,
                }
            },
            use_gamemode:  if cfg!(windows) {
                None
            } else {
                match value.get("wine_binary_path") {
                    Some(value) => match value.as_bool() {
                        Some(value) => Some(value),
                        None => default.use_gamemode,
                    },
                    None => default.use_gamemode,
                }
            },
            use_dxvk_async:  if cfg!(windows) {
                None
            } else {
                match value.get("use_dxvk_async") {
                    Some(value) => match value.as_bool() {
                        Some(value) => Some(value),
                        None => default.use_dxvk_async,
                    },
                    None => default.use_dxvk_async,
                }
            },
            use_esync:  if cfg!(windows) {
                None
            } else {
                match value.get("use_esync") {
                    Some(value) => match value.as_bool() {
                        Some(value) => Some(value),
                        None => default.use_esync,
                    },
                    None => default.use_esync,
                }
            },
            use_fsync:  if cfg!(windows) {
                None
            } else {
                match value.get("use_fsync") {
                    Some(value) => match value.as_bool() {
                        Some(value) => Some(value),
                        None => default.use_fsync,
                    },
                    None => default.use_fsync,
                }
            },
            use_futex2:  if cfg!(windows) {
                None
            } else {
                match value.get("use_futex2") {
                    Some(value) => match value.as_bool() {
                        Some(value) => Some(value),
                        None => default.use_futex2,
                    },
                    None => default.use_futex2,
                }
            },
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ProgramEntry {
    enabled: bool,
    program: String,
}

impl From<&Value> for ProgramEntry {
    fn from(value: &Value) -> Self {
        let default = Self::default();

        Self {
            enabled: match value.get("enabled") {
                Some(value) => match value.as_bool() {
                    Some(value) => value,
                    None => default.enabled,
                },
                None => default.enabled,
            },
            program: match value.get("program") {
                Some(value) => match value.as_str() {
                    Some(value) => value.to_string(),
                    None => default.program,
                },
                None => default.program,
            },
        }
    }
}
