use crate::capture::Response;
use chrono::prelude::*;
use tauri::{Window, AppHandle, TitleBarStyle, Manager, PhysicalSize, PhysicalPosition};
use std::path::Path;



mod capture;

#[tauri::command]
async fn folder_dialog(handle: AppHandle) -> Response {
    capture::folder_dialog(handle).await
}

#[tauri::command]
async fn cuhd() -> Response {
    capture::cuhd()
}


#[tauri::command(rename_all = "snake_case")]
async fn capture(window: Window, mode: &str, view: &str, area: &str, timer: u64, pointer: bool, file_path: &str, file_type: &str, clipboard: bool, audio: bool, open_file: bool) -> Result<Response, String> {
    let abs_path: String;
    let fs_path = Path::new(file_path);

    if file_path.ends_with("/") {
        if !fs_path.exists() {
            return Err(format!("{} is not a valid path.", file_path));
        }
        let current_date = Local::now();
        let formatted_date = current_date.format("%Y-%m-%d at %H-%M-%S").to_string();
        abs_path = format!("{}Screen Crab {}.{}", file_path, formatted_date, file_type);
    }
    else {
        abs_path = format!("{}.{}", file_path, file_type);
    }


    match mode {
        "capture" => {
            match view {
                "fullscreen" => {
                    #[cfg(target_os = "macos")]
                    return Ok(capture::capture_fullscreen(window, abs_path.as_str(), &file_type, timer, pointer, clipboard, open_file).await);
                }
                "custom" => {
                    #[cfg(target_os = "macos")]
                    return Ok(capture::capture_custom(window, area, abs_path.as_str(), &file_type, timer, pointer, clipboard, open_file).await);

                }
                _ => return Ok(Response::new(None, Some(format!("Invalid view: {}", view))))
            }
        }
        "record" => {
            match view {
                "fullscreen" => {
                    #[cfg(target_os = "macos")]
                    return Ok(capture::record_fullscreen(window, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await);
                }
                "custom" => {
                    #[cfg(target_os = "macos")]
                    return Ok(capture::record_custom(window, area, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await);
                }
                _ => return Ok(Response::new(None, Some(format!("Invalid view: {}", view))))
            }
             }
        _ => return Ok(Response::new(None, Some(format!("Invalid mode: {}", mode))))
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let monitor_size = *app.get_window("main").unwrap().current_monitor().unwrap().unwrap().size();
            let width = monitor_size.width*4/5;
            let height = monitor_size.height*8/30;
            app.handle().windows().get("main").unwrap().set_size(PhysicalSize::new(width, height)).unwrap();
            app.handle().windows().get("main").unwrap().set_position(PhysicalPosition::new((monitor_size.width-width)/2, monitor_size.height-height*14/10)).unwrap();
            let area = tauri::WindowBuilder::new(
                app,
                "selector",
                tauri::WindowUrl::App("./blank.html".into()))
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
            area.set_size(PhysicalSize::new(width/2, height)).unwrap();
            area.hide().unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![capture, folder_dialog, cuhd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}