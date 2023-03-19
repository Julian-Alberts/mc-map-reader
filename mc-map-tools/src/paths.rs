use std::{io::Write, path::PathBuf};

pub enum Files {
    PluginSettings,
    ConfigFile,
    LogFile,
}

impl Files {
    pub fn path(&self) -> PathBuf {
        let mut path = get_config_dir();
        match self {
            Files::PluginSettings => {
                path.push("plugins.json");
            }
            Files::ConfigFile => {
                path.push("config.json");
            }
            Files::LogFile => {
                path.push("error.log");
            }
        }
        path
    }
}

impl From<Files> for PathBuf {
    fn from(file: Files) -> Self {
        file.path()
    }
}

pub enum Directories {
    Plugins,
    Base,
}

impl Directories {
    pub fn path(&self) -> PathBuf {
        match self {
            Directories::Plugins => get_plugin_dir(),
            Directories::Base => get_config_dir(),
        }
    }
}

impl From<Directories> for PathBuf {
    fn from(dir: Directories) -> Self {
        dir.path()
    }
}

fn get_config_dir() -> PathBuf {
    let mut home = if cfg!(debug_assertions) {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    } else {
        dirs::config_dir().expect("$HOME is not set")
    };
    home.push("ja-mc-map-tools");
    home
}

fn get_plugin_dir() -> PathBuf {
    let mut plugin_path = get_config_dir();
    plugin_path.push("plugins");
    plugin_path
}

#[allow(unused)]
fn init() -> std::io::Result<()> {
    use std::fs::create_dir_all;
    create_dir_all(get_plugin_dir())?;
    if !Files::PluginSettings.path().exists() {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(Files::PluginSettings.path())?;
        f.write_all(b"{}")?;
    }
    Ok(())
}
