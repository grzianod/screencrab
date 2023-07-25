use std::ffi::OsString;
use crate::lib::Response;
use chrono::prelude::*;

mod lib;

#[tauri::command]
async fn folder_dialog() -> Response {
    lib::folder_dialog().await
}

#[tauri::command]
async fn cwd() -> Response {
    lib::cwd().await
}


#[tauri::command(rename_all = "snake_case")]
fn capture(mode: String, view: String, pointer: bool, path: String, name: String, file_type: String) {
    let filename: String;
    if name.is_empty() {
        let current_date = Local::now();
        let formatted_date = current_date.format("%H-%M-%S on %Y-%m-%d").to_string();
        filename = format!("{}/Screen Crab at {}{}", path.as_str(), formatted_date, file_type);
    }
    else {
        filename = format!("{}/{}{}", path.as_str(), name.as_str(), file_type);
    }
    lib::capture_screen(filename.as_str()).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![capture, folder_dialog, cwd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
