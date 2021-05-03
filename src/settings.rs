use config::{ConfigError, Config, File};

#[derive(Debug, Deserialize)]
pub struct Daemon {
    pub name: String,
    pub workdir: String,
    pub user: String,
    pub group: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub daemon: Daemon,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        match s.merge(File::with_name("default")) {
            Err(e) => return Err(e),
            _ => ()
        }
        s.try_into()
    }
}
