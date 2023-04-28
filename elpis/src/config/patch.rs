

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json::Value;

pub mod prelude {
    pub use super::Patch;
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Patch {
    pub path: PathBuf,
    pub keep_patches: bool,
    pub download_method: DownloadMethod,
    pub download_speed_limit: i64,
}

impl Default for Patch {
    fn default() -> Self {
        let launcher_dir = crate::consts::launcher_dir();
        Self {
            path: launcher_dir.join("patches"),
            keep_patches: false,
            download_method: DownloadMethod::Native,
            download_speed_limit: 1000,
        }
    }
}

impl From<&Value> for Patch {
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
            keep_patches: match value.get("patches") {
                Some(value) => match value.as_bool() {
                    Some(value) => value,
                    None => default.keep_patches,
                },
                None => default.keep_patches,
            },
            download_method: match value.get("download_method") {
                Some(value) => DownloadMethod::from(value),
                None => default.download_method,
            },
            download_speed_limit: match value.get("download_speed_limit") {
                Some(value) => match value.as_i64() {
                    Some(value) => value,
                    None => default.download_speed_limit,
                },
                None => default.download_speed_limit,
            }
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum DownloadMethod {
    #[default]
    Native = 0,
    Aria = 1,
    Torrent = 2,
}

impl From<&Value> for DownloadMethod {
    fn from(value: &Value) -> Self {
        serde_json::from_value(value.clone()).unwrap_or_default()
    }
}
