use crate::capture::Response;
use chrono::prelude::*;
use tauri::{Window, AppHandle, TitleBarStyle, PhysicalSize, PhysicalPosition};
use std::path::Path;
use crate::menu::create_context_menu;
use tauri::{Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

mod capture;
mod menu;

#[tauri::command]
async fn folder_dialog(handle: AppHandle) -> Response {
    capture::folder_dialog(handle).await
}

#[tauri::command]
async fn cuhd() -> Response {
    capture::cuhd()
}


#[tauri::command(rename_all = "snake_case")]
async fn capture(app: AppHandle, window: Window, mode: &str, view: &str, area: &str, timer: u64, pointer: bool, file_path: &str, file_type: &str, clipboard: bool, open_file: bool) -> Result<Response, String> {
    let abs_path: String;
    let fs_path = Path::new(file_path);

    if !fs_path.exists() || !fs_path.is_dir() {
        return Err(format!("\"{}\" is not a valid directory.", file_path));
    }

    if file_path.ends_with("/") {
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
                    return Ok(capture::capture_fullscreen(app, window, abs_path.as_str(), &file_type, timer, pointer, clipboard, open_file).await);
                }
                "custom" => {
                    #[cfg(target_os = "macos")]
                    return Ok(capture::capture_custom(app, window, area, abs_path.as_str(), &file_type, timer, pointer, clipboard, open_file).await);

                }
                _ => return Ok(Response::new(None, Some(format!("Invalid view: {}", view))))
            }
        }
        "record" => {
            match view {
                "fullscreen" => {
                    #[cfg(target_os = "macos")]
                    return Ok(capture::record_fullscreen(app, window, abs_path.as_str(), timer, pointer, clipboard,  open_file).await);
                }
                "custom" => {
                    #[cfg(target_os = "macos")]
                    return Ok(capture::record_custom(app, window, area, abs_path.as_str(), timer, pointer, clipboard,  open_file).await);
                }
                _ => return Ok(Response::new(None, Some(format!("Invalid view: {}", view))))
            }
             }
        _ => return Ok(Response::new(None, Some(format!("Invalid mode: {}", mode))))
    }
}

fn main() {
    let system_tray_menu = SystemTrayMenu::new();
    tauri::Builder::default()
        .menu(create_context_menu())
        .on_menu_event(|event| {
            event.window().emit_all(event.menu_item_id(), {}).unwrap();
            match event.menu_item_id() {
                "capture_mouse_pointer" => {
                    event.window().menu_handle().get_item(event.menu_item_id()).set_selected(true).unwrap();
                }
                _ => {}
            }
        })
        .setup(|app| {
            let monitor_size = *app.get_window("main").unwrap().current_monitor().unwrap().unwrap().size();
            let width = monitor_size.width*28/40;
            let height = monitor_size.height*4/15;
            app.handle().windows().get("main").unwrap().set_size(PhysicalSize::new(width, height)).unwrap();
            app.handle().windows().get("main").unwrap().set_position(PhysicalPosition::new((monitor_size.width-width)/2, monitor_size.height-height*14/10)).unwrap();
            let area = tauri::WindowBuilder::new(
                app,
                "selector",
                tauri::WindowUrl::App("./blank.html".into()))
                .decorations(false)
                .title_bar_style(TitleBarStyle::Overlay)
                .transparent(true)
                .resizable(true)
                .always_on_top(true)
                .center()
                .title("")
                .content_protected(true)
                .focused(true)
                .build().unwrap();
            area.set_size(PhysicalSize::new(width/2, height)).unwrap();
            area.hide().unwrap();

            Ok(())
        })
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                // toggle application window
                if window.is_visible().unwrap() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            },
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![capture, folder_dialog, cuhd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}