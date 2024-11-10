use futures_util::future::ok;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, sync::Arc, sync::Mutex};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub project_dir: String,
    pub blog_content: String,
}

#[derive(Debug, Default)]
pub struct ConfigManager {
    pub config: Option<Config>,
    path: String,
}

impl ConfigManager {
    pub fn new(path: String) -> ConfigManager {
        ConfigManager {
            config: None,
            path: path,
        }
    }

    pub fn load_conf(&mut self) -> Result<(), String> {
        if self.config.is_some() {
            return Ok(());
        }

        let res = fs::read_to_string(&self.path).map_err(|err| err.to_string())?;
        self.config = Some(toml::from_str(&res).map_err(|err| err.to_string())?);
        Ok(())
    }

    pub fn save(&mut self, content: &str) -> Result<(), String> {
        let conf_raw = toml::from_str::<toml::Value>(content).map_err(|err| err.to_string())?;
        if let toml::Value::Table(table) = &conf_raw {
            if !table.contains_key("project_dir") {
                return Err("project_dir 必须存在".to_string());
            }
        }

        let mut file = fs::File::create(&self.path).map_err(|err| err.to_string())?;
        file.write_all(content.as_bytes())
            .map_err(|err| err.to_string());

        self.load_conf();

        Ok(())
    }
}

lazy_static! {
    static ref CONFIG_MANAGER: Arc<Mutex<ConfigManager>> =
        Arc::new(Mutex::new(ConfigManager::new("conf/hugo.toml".to_string())));
}

pub fn get_config_manager() -> Arc<Mutex<ConfigManager>> {
    CONFIG_MANAGER.clone()
}
