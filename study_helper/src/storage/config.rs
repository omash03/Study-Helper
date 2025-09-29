use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use std::fs;
use std::io;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Config {
    pub storage_base_path: String,
    pub storage_class_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            storage_base_path: String::new(),
            storage_class_name: String::new(),
        }
    }
}

/// Path to the config file. Currently placed next to the executable / working directory as `config.json`.
fn config_path() -> PathBuf {
    // Prefer a config file located next to the running executable. When the app
    // is launched via a Windows shortcut the working directory can differ
    // from the executable directory ("Start in" setting), so using the
    // executable directory avoids "config not found" issues.
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(dir) = exe_path.parent() { 
            return dir.join("config.json");
        }
    }

    // Fallback: use current working directory (preserves previous behavior if
    // current_exe() fails for some reason).
    std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf()).join("config.json")
}

/// Load configuration from disk. If the file doesn't exist, returns `Config::default()`.
pub fn load_config() -> io::Result<Config> {
    let p = config_path();
    if !p.exists() {
        return Ok(Config::default());
    }
    let s = fs::read_to_string(&p)?;
    let cfg: Config = serde_json::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(cfg)
}

/// Save the provided config to disk (overwrites).
pub fn save_config(cfg: &Config) -> io::Result<()> {
    let s = serde_json::to_string_pretty(cfg).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let p = config_path();
    if let Some(parent) = p.parent() {
        // parent is typically the current directory; attempt to create if needed
        let _ = fs::create_dir_all(parent);
    }
    fs::write(&p, s)?;
    Ok(())
}
