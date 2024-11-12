// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;

use tauri::{AppHandle, Builder, WindowEvent};
use warp::Filter;
mod setup;
use setup::{conf, ws};
use tauri_plugin_shell::ShellExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn update_conf(content: &str) -> Result<(), String> {
    let global_conf = conf::get_config_manager();
    let mut conf = global_conf.lock().await;
    conf.save(content)
}

#[tauri::command]
async fn load_conf() -> String {
    let global_conf = conf::get_config_manager();
    let mut project_conf = global_conf.lock().await;
    match project_conf.load_conf() {
        Ok(_) => {
            if let Some(ref obj) = project_conf.config {
                let result = match toml::to_string(&obj) {
                    Ok(toml_str) => {
                        return toml_str;
                    }
                    Err(err) => {
                        return err.to_string();
                    }
                };
            } else {
                return "对象解析失败".to_string();
            }
        }
        Err(e) => {
            return e;
        }
    }
}

#[tauri::command]
fn close_hugo() -> String {
    // 模拟从数据库或文件中获取数据
    close_hugo_start_pid();
    "关闭hugo".to_string()
}

#[tauri::command]
async fn open_blog(app_handle: AppHandle, application: String) {
    let shell = app_handle.shell();

    // 通过配置文件获取博客内容的路径
    let global_conf_manager = conf::get_config_manager().clone();
    let project_conf = global_conf_manager.lock().await;
    if let Some(ref info) = project_conf.config {
        let blog_content_path = format!("{}", info.blog_content);
        let output = shell
            .command("sh")
            .arg("-c")
            .arg(format!("open -a {} {}", application, blog_content_path).to_string()) // 硬编码
            .output()
            .await;
    }
}

#[tauri::command]
async fn get_applications(app_handle: AppHandle) -> Vec<String> {
    let shell = app_handle.shell();
    let output = shell
        .command("sh")
        .arg("-c")
        .arg("ls /Applications/ | awk -F '.' '{print $1}'")
        .output()
        .await
        .unwrap();
    if output.status.success() {
        if let Ok(lines) = String::from_utf8(output.stdout) {
            let res: Vec<String> = lines
                .split('\n')
                .filter(|s| !s.is_empty()) // 过滤掉空字符串
                .map(String::from) // 将每个 &str 转换为 String
                .collect();
            return res;
        }
    } else {
        println!("Exit with code: {}", output.status.code().unwrap());
    }

    vec!["Obsidian".to_string(), "GoLand".to_string()]
}

#[tokio::main]
async fn main() {
    tokio::spawn(async move {
        // GET /chat -> websocket upgrade
        let chat = warp::path("chat")
            // The `ws()` filter will prepare Websocket handshake...
            .and(warp::ws())
            .map(|ws: warp::ws::Ws| {
                // This will call our function if the handshake succeeds.
                ws.on_upgrade(move |socket| ws::user_connected(socket))
            });

        warp::serve(chat).run(([127, 0, 0, 1], 3030)).await;
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            close_hugo,
            load_conf,
            update_conf,
            get_applications,
            open_blog
        ])
        .on_window_event(|window, event| match event {
            WindowEvent::CloseRequested { api, .. } => {
                close_hugo_start_pid();
                println!("CloseRequested")
            }
            WindowEvent::Destroyed => {
                close_hugo_start_pid();
                println!("destroy")
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn close_hugo_start_pid() {
    // 在这里执行 shell 脚本
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("lsof -i :1313 | grep -i LISTEN | awk '{print $2}'")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // 解析输出，获取进程 ID
    let pids: Vec<i32> = stdout
        .split_whitespace()
        .filter_map(|pid_str| pid_str.parse::<i32>().ok())
        .collect();

    // 终止进程 (示例：使用 kill 命令，请注意安全性和权限问题)
    for pid in pids {
        std::process::Command::new("kill")
            .arg("-9")
            .arg(pid.to_string())
            .output()
            .expect("failed to kill process");
    }
}
