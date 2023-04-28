
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use cached::once_cell::sync::Lazy;

use serde::{Deserialize, Serialize};

pub mod boot;
pub mod game;
pub mod patch;

pub mod prelude {
    pub use super::boot::prelude::*;
    pub use super::game::prelude::*;
    pub use super::patch::prelude::*;
}

use prelude::*;

static mut CONFIG: Option<Config> = None;

#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub boot: Boot,
    pub game: Game,
    pub patch: Patch,
}

pub fn get() -> anyhow::Result<Config> {
    unsafe {
        match &CONFIG {
            Some(config) => Ok(config.clone()),
            None => get_raw(),
        }
    }
}

pub fn get_raw() -> anyhow::Result<Config> {
    tracing::debug!("Reading config file");
    let path = crate::config_file();

    if path.exists() && path.is_file() {
        let file = ::std::fs::read_to_string(&path)?;
        let config = serde_json::from_str::<Config>(&file)?;
        update(config.clone());
        Ok(config)
    } else {
        update_raw(Config::default())?;
        Ok(Config::default())
    }
}

pub fn update(config: Config) {
    unsafe {
        CONFIG = Some(config);
    }
}

pub fn update_raw(config: Config) -> anyhow::Result<()> {
    tracing::debug!("Updating config file");
    update(config.clone());
    let json = serde_json::to_string_pretty(&config)?;
    ::std::fs::write(crate::config_file(), json)?;
    Ok(())
}

pub fn flush() -> anyhow::Result<()> {
    tracing::debug!("Resetting config to previous state");
    update_raw(get()?)
}