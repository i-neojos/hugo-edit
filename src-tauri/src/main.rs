// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
