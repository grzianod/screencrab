use crate::capture::Response;
use chrono::prelude::*;
use tauri::Manager;
use tauri::Window;
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
async fn capture(window: Window, mode: &str, view: &str, timer: u64, pointer: bool, path: &str, name: &str, file_type: &str, clipboard: bool) -> Result<Response, ()> {
    let file: String;
    if name.is_empty() {
        let current_date = Local::now();
        let formatted_date = current_date.format("%Y-%m-%d at %H-%M-%S").to_string();
        file = format!("{}/Screen Crab {}.{}", path, formatted_date, file_type);
    } else {
        file = format!("{}/{}.{}", path, name, file_type);
    }

        match mode {
            "capture" => {
                #[cfg(target_os = "macos")]
                Ok(capture::capture_screen(window, file.as_str(), &file_type, &view, timer, pointer, clipboard).await)
            }
            "record" => {
                #[cfg(target_os = "macos")]
                Ok(capture::record_screen(file.as_str()).await)
            }
            _ => Ok(Response::new(None, Some(format!("Invalid mode: {}", mode)))),
        }

    }


#[tauri::command]
async fn kill(pid: u32) -> Response {
    capture::kill(pid).await
}

fn main() {

    tauri::Builder::default()
        .setup(|app| {
            // listen to the `stop` event (emitted on any window)
            app.listen_global("kill", |event| {
                println!("got kill with payload {:?}", event.payload());
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![capture, folder_dialog, cwd, kill])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}