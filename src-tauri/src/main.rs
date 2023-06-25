// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod selenium;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tokio::main]
async fn main() {
    let selenium_state = selenium::Selenium::new();

    tauri::Builder::default()
        .manage(selenium_state)
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .unwrap_or_else(|_| {
            panic!("error while running tauri application")
        });
}
