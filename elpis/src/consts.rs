use std::path::PathBuf;

use cached::proc_macro::cached;
use sha1::{Sha1, Digest};

#[cached]
pub fn launcher_dir() -> anyhow::Result<PathBuf> {
    Ok(::std::env::var("XDG_DATA_HOME")
        .or_else(|_| ::std::env::var("HOME").map(|dir| dir + "/.local/share"))
        .map(|dir| PathBuf::from(dir).join("xivlite"))?)
}

#[cached]
pub fn config_file() -> PathBuf {
    launcher_dir().map(|dir| dir.join("config.json"))
}

#[cached]
pub fn host_id() -> String {
    let mut host_id = String::new();
    host_id.push_str(&whoami::hostname());
    host_id.push_str(&whoami::username());
    host_id.push_str(&os_version::detect().map(|v| v.to_string()).unwrap_or(String::from("")));
    host_id.push_str(&num_cpus::get().to_string());

    let mut hasher = Sha1::default();
    hasher.update(&host_id);
    let hash = hasher.finalize();
    let mut bytes = vec![0u8; 5];
    (0..bytes.len() - 1)
        .for_each(|i| bytes[i + 1] = hash[i]);
    let checksum = bytes[1] + bytes[2] + bytes[3] + bytes[4];
    bytes[0] = checksum;

    hex::encode(bytes)
}