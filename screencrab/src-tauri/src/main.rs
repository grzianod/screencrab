use crate::capture::Response;
use chrono::prelude::*;
use tauri::{Window, AppHandle, TitleBarStyle};


mod capture;

#[tauri::command]
async fn folder_dialog(handle: AppHandle) -> Response {
    capture::folder_dialog(handle).await
}

#[tauri::command]
async fn cwd() -> Response {
    capture::cwd().await
}


#[tauri::command(rename_all = "snake_case")]
async fn capture(window: Window, mode: &str, view: &str, area: &str, timer: u64, pointer: bool, path: &str, name: &str, file_type: &str, clipboard: bool) -> Result<Response, ()> {
    let file: String;
    if name.is_empty() {
        let current_date = Local::now();
        let formatted_date = current_date.format("%Y-%m-%d at %H-%M-%S").to_string();
        if path.eq("/") {
            file = format!("{}Screen Crab {}.{}", path, formatted_date, file_type);
        }
        else {
            file = format!("{}/Screen Crab {}.{}", path, formatted_date, file_type);
        }
    } else {
        file = format!("{}/{}.{}", path, name, file_type);
    }


    match mode {
        "capture" => {
            match view {
                "fullscreen" => {
                    #[cfg(target_os = "macos")]
                    Ok(capture::capture_fullscreen(window, file.as_str(), &file_type, timer, pointer, clipboard).await)
                }
                "custom" => {
                    #[cfg(target_os = "macos")]
                    Ok(capture::capture_custom(window, area, file.as_str(), &file_type, timer, pointer, clipboard).await)

                }
                _ => Ok(Response::new(None, Some(format!("Invalid view: {}", view))))
            }
        }
        "record" => {
            match view {
                "fullscreen" => {
                    #[cfg(target_os = "macos")]
                    Ok(capture::record_fullscreen(window, file.as_str(), timer, pointer, clipboard).await)
                }
                "custom" => {
                    #[cfg(target_os = "macos")]
                    Ok(capture::record_custom(window, area, file.as_str(), timer, pointer, clipboard).await)
                }
                _ => Ok(Response::new(None, Some(format!("Invalid view: {}", view))))
            }
             }
        _ => Ok(Response::new(None, Some(format!("Invalid mode: {}", mode))))
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let area = tauri::WindowBuilder::new(
                app,
                "selector",
                tauri::WindowUrl::App("./blank.html".into()))
                .inner_size(300.0, 300.0)
                .decorations(false)
                .title_bar_style(TitleBarStyle::Overlay)
                .always_on_top(true)
                .transparent(true)
                .resizable(true)
                .center()
                .skip_taskbar(true)
                .title("")
                .content_protected(true)
                .focused(true)
                .build().unwrap();
            area.hide().unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![capture, folder_dialog, cwd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}