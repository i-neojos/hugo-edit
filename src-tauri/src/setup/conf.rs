use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, sync::Arc, sync::Mutex};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub project_dir: String,
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
        let res = fs::read_to_string(&self.path).map_err(|err| err.to_string())?;
        self.config = Some(toml::from_str(&res).map_err(|err| err.to_string())?);
        Ok(())
    }

    pub fn save(&self, content: &str) -> Result<(), String> {
        let conf_raw = toml::from_str::<toml::Value>(content).map_err(|err| err.to_string())?;
        if let toml::Value::Table(table) = &conf_raw {
            if !table.contains_key("project_dir") {
                return Err("project_dir 必须存在".to_string());
            }
        }

        let mut file = fs::File::create(&self.path).map_err(|err| err.to_string())?;
        file.write_all(content.as_bytes())
            .map_err(|err| err.to_string());
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

pub fn load_conf() -> Result<String, String> {
    let path = "conf/hugo.toml";
    let res = match fs::read_to_string(path) {
        Ok(contents) => Ok(contents),

        Err(e) => Err(format!("read conf error {}", e)),
    };
    return res;
}

pub fn update_conf(content: String) -> String {
    let conf_raw = toml::from_str::<toml::Value>(&content);
    match conf_raw {
        Ok(value) => {
            if let toml::Value::Table(table) = &value {
                if !table.contains_key("project_path") {
                    return "project_path 必须存在".to_string();
                }
            }
        }
        Err(e) => {
            return "非有效的toml格式".to_string();
        }
    };

    let file_path = "conf/hugo.toml";
    let mut file = fs::File::create(file_path);
    match file {
        Ok(mut f) => {
            f.write_all(content.as_bytes());
        }
        Err(e) => {
            return format!("write file error {}", e);
        }
    };

    return "success".to_string();
}
