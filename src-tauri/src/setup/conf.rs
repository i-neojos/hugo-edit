use std::{fs, io::{Write}};
use toml;

pub fn load_conf() -> Result<String, String> {
    let path = "conf/hugo.toml"; 
    let res = match fs::read_to_string(path) {
        Ok(contents) => Ok(contents),
        Err(e) => Err(format!("read conf error {}", e))
    };
    return res;
}

pub fn update_conf(content: String) -> String {

    let conf_raw = toml::from_str::<toml::Value>(&content);
    match conf_raw {
        Ok(value) => {
            if let toml::Value::Table(table) = &value {
                if !table.contains_key("project_path") {
                    return  "project_path 必须存在".to_string();
                }
            }
        },
        Err(e) => {
            return  "非有效的toml格式".to_string();
        }
    };

    let file_path = "conf/hugo.toml"; 
    let mut file = fs::File::create(file_path);
    match file {
        Ok(mut f) => {
            f.write_all(content.as_bytes());
        },
        Err(e) => {return format!("write file error {}", e);}
    };

    return "success".to_string();
}