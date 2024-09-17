// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::WindowEvent;
use warp::Filter;
mod setup;
use setup::ws;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn dump_content() -> String {
    // 模拟从数据库或文件中获取数据
    "这是一次特别简单的交互".to_string()
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
        .invoke_handler(tauri::generate_handler![greet, dump_content])
        .on_window_event(|event| {
            match event.event() {
                WindowEvent::CloseRequested { api, .. } => {
                    get_hugo_start_pid();
                    println!("CloseRequested")
                },
                WindowEvent::Destroyed => {
                    get_hugo_start_pid();
                    println!("destroy")
                }
                _ => {
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_hugo_start_pid() {
    // 在这里执行 shell 脚本
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("lsof -i :1313 | grep -i LISTEN | awk '{print $2}'")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // 解析输出，获取进程 ID
    let pids: Vec<i32> = stdout.split_whitespace()
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