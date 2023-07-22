// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lib;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn cwd() -> String {
    lib::cwd()
}


#[tauri::command]
fn capture() {
    let filename = "screenshot.png";
    lib::capture_screen(filename).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![capture, cwd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
