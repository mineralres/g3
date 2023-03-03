#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use ctp_futures::trader_api::*;
use log::info;
use tauri::Manager;

pub fn init_logger() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::init();
}

#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    info!("close_splashscreen");
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    window.get_window("main").unwrap().show().unwrap();
}

// Register the command:
#[tokio::main]
async fn main() {
    init_logger();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![close_splashscreen])
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
