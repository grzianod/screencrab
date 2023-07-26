use crate::capture::Response;
use chrono::prelude::*;

mod capture;

#[tauri::command]
async fn folder_dialog() -> Response {
    capture::folder_dialog().await
}

#[tauri::command]
async fn cwd() -> Response {
    capture::cwd().await
}


#[tauri::command(rename_all = "snake_case")]
fn capture(mode: &str, view: &str, pointer: bool, path: &str, name: &str, file_type: &str, clipboard: bool) {
    let file: String;
    if name.is_empty() {
        let current_date = Local::now();
        let formatted_date = current_date.format("%Y-%m-%d at %H-%M-%S").to_string();
        file = format!("{}/Screen Crab {}.{}", path, formatted_date, file_type);
    }
    else {
        file = format!("{}/{}.{}", path, name, file_type);
    }
    match mode {
        "capture" => {
            capture::capture_screen(file.as_str(), file_type, view, pointer, clipboard).unwrap();
        }
        "record" => {
            capture::record_screen(file.as_str()).unwrap();
        }
        _ => println!("Error occurred") /* TODO: handle error */
    }

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![capture, folder_dialog, cwd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
