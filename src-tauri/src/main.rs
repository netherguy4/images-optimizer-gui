#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod image_ops;
mod optimizer;
mod tools;
mod types;

use moka::future::Cache;
use std::sync::Mutex;
use std::time::Duration;

use image_ops::{generate_thumbnail, ImageCache};
use commands::run_optimization;
use types::AppState;

fn main() {
    let cache = Cache::builder()
        .max_capacity(500)
        .time_to_idle(Duration::from_secs(30 * 60))
        .build();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            is_processing: Mutex::new(false),
        })
        .manage(ImageCache(cache))
        .invoke_handler(tauri::generate_handler![
            run_optimization,
            generate_thumbnail
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
