
#![cfg_attr(all(not(debug_assertions), target_os = "macos"), windows_subsystem = "console")]

use chrono::prelude::*;
use tauri::{Window, AppHandle, PhysicalSize, PhysicalPosition};
use std::path::Path;
use crate::menu::{create_context_menu};
use tauri::{Manager, SystemTray, SystemTrayEvent};
use std::cell::Cell;



mod menu;


use std::{env, fs};
use serde_json;

#[derive(serde::Deserialize)]
struct HotkeyInput {
    hotkeyData: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct CmdArgs {
    message: String,
}

#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
#[cfg(target_os = "macos")]
mod darwin;
#[cfg(target_os = "macos")]
use crate::darwin::Response;


#[cfg(target_os = "windows")]
    mod windows;
#[cfg(target_os = "windows")]
    use crate::windows::Response;


#[cfg(target_os = "linux")]
    mod linux;

#[cfg(target_os = "linux")]
    use crate::linux::Response;


#[tauri::command]
async fn folder_dialog(handle: AppHandle) -> Response {
    #[cfg(target_os = "macos")]
    return darwin::folder_dialog(handle).await;
    #[cfg(target_os = "linux")]
    return linux::folder_dialog(handle).await;
}

#[tauri::command]
async fn current_default_path() -> Response {
    #[cfg(target_os = "macos")]
    return darwin::current_default_path().await;
    #[cfg(target_os = "linux")]
    return linux::current_default_path().await;
}


#[tauri::command(rename_all = "snake_case")]
async fn capture(app: AppHandle, window: Window, mode: &str, view: &str, area: &str, timer: u64, pointer: bool, file_path: &str, file_type: &str, clipboard: bool, audio: bool, open_file: bool) -> Result<Response, String> {
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
                    return Ok(darwin::capture_fullscreen(app, window, abs_path.as_str(), &file_type, timer, pointer, clipboard, audio, open_file).await);
                    #[cfg(target_os = "linux")]
                    return Ok(linux::capture_fullscreen(app, window, abs_path.as_str(), &file_type, timer, pointer, clipboard, audio, open_file).await);
                    #[cfg(target_os = "windows")]
                    return Ok(windows::capture_fullscreen(app, window, abs_path.as_str(), &file_type, timer, pointer, clipboard, audio, open_file).await);
                }
                "custom" => {
                    #[cfg(target_os = "macos")]
                    return Ok(darwin::capture_custom(app, window, area, abs_path.as_str(), &file_type, timer, pointer, clipboard, audio, open_file).await);
                    #[cfg(target_os = "linux")]
                    return Ok(linux::capture_custom(app, window, area, abs_path.as_str(), &file_type, timer, pointer, clipboard, audio, open_file).await);
                    #[cfg(target_os = "windows")]
                    return Ok(windows::capture_custom(app, window, area, abs_path.as_str(), &file_type, timer, pointer, clipboard, audio, open_file).await);
                }
                _ => return Ok(Response::new(None, Some(format!("Invalid view: {}", view))))
            }
        }
        "record" => {
            match view {
                "fullscreen" => {
                    #[cfg(target_os = "macos")]
                    return Ok(darwin::record_fullscreen(app, window, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await);
                    #[cfg(target_os = "linux")]  
                    return Ok(linux::record_fullscreen(app, window, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await);
                    #[cfg(target_os = "windows")]
                    return Ok(windows::record_fullscreen(app, window, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await);
                }
                "custom" => {
                    #[cfg(target_os = "macos")]
                    return Ok(darwin::record_custom(app, window, area, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await);
                    #[cfg(target_os = "linux")] 
                    return Ok(linux::record_custom(app, window, area, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await);
                    #[cfg(target_os = "windows")]
                    return Ok(windows::record_custom(app, window, area, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await);
                }
                _ => return Ok(Response::new(None, Some(format!("Invalid view: {}", view))))
            }
             }
        _ => return Ok(Response::new(None, Some(format!("Invalid mode: {}", mode))))
    }
}

#[tauri::command]
fn get_home_dir() -> String {
    env::var("HOME").unwrap()
}

#[tauri::command]
async fn load_hotkeys() -> String {
    menu::hotkeys()
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {

            #[cfg(target_os="macos")]
            let area = tauri::WindowBuilder::new(
                app,
                "selector",
                tauri::WindowUrl::App("./blank.html".into()))
                .title_bar_style(TitleBarStyle::Overlay)
                .decorations(false)
                .transparent(true)
                .resizable(true)
                .always_on_top(true)
                .center()
                .title("")
                .content_protected(true)
                .always_on_top(true)
                .minimizable(false)
                .focused(true)
                .build()
                .unwrap();


            #[cfg(target_os="linux")]
            let area = tauri::WindowBuilder::new(
                app,
                "selector",
                tauri::WindowUrl::App("./blank.html".into()))
                .decorations(false)
                .transparent(true)
                .resizable(true)
                .always_on_top(true)
                .center()
                .title("")
                .content_protected(true)
                .always_on_top(true)
                .minimizable(false)
                .focused(true)
                .build()
                .unwrap();

                let monitor_size = area.current_monitor().unwrap().unwrap().size().to_owned();
                let width = monitor_size.width*60/100;
                let height = monitor_size.height*23/100;

                area.set_size(PhysicalSize::new(width/2, height)).unwrap();
                area.hide().unwrap();

            let main_window = tauri::WindowBuilder::new(
                app,
                "main_window",
                tauri::WindowUrl::App("./index.html".into()))
                .menu(create_context_menu())
                .fullscreen(false)
                .resizable(false)
                .closable(true)
                .minimizable(true)
                .focused(true)
                .title("Screen Crab")
                .content_protected(true)
                .decorations(true)
                .build()
                .unwrap();

                main_window.set_size(PhysicalSize::new(width, height)).unwrap();
                main_window.set_position(PhysicalPosition::new((monitor_size.width-width)/2, monitor_size.height-height*16/10)).unwrap();

                let capture_mouse_pointer = Cell::new(false);
                let copy_to_clipboard = Cell::new(false);
                let edit_after_capture = Cell::new(true);
                let record_external_audio = Cell::new(false);
                let open_after_record = Cell::new(true);

                let window_ = main_window.clone();
                main_window.on_menu_event(move |event| {
                    match event.menu_item_id() {
                        "capture_mouse_pointer" => {
                            capture_mouse_pointer.set(!capture_mouse_pointer.get());
                            window_.menu_handle().get_item(event.menu_item_id()).set_selected(capture_mouse_pointer.get()).unwrap();
                        }
                        "copy_to_clipboard" => {
                            copy_to_clipboard.set(!copy_to_clipboard.get());
                                edit_after_capture.set(!copy_to_clipboard.get());
                                window_.menu_handle().get_item("edit_after_capture").set_selected(!copy_to_clipboard.get()).unwrap();
                                window_.menu_handle().get_item("edit_after_capture").set_enabled(!copy_to_clipboard.get()).unwrap();

                            window_.menu_handle().get_item(event.menu_item_id()).set_selected(copy_to_clipboard.get()).unwrap();
                        }
                        "edit_after_capture" => {
                            edit_after_capture.set(!edit_after_capture.get());
                            window_.menu_handle().get_item(event.menu_item_id()).set_selected(edit_after_capture.get()).unwrap();
                        }
                        "record_external_audio" => {
                            record_external_audio.set(!capture_mouse_pointer.get());
                            window_.menu_handle().get_item(event.menu_item_id()).set_selected(record_external_audio.get()).unwrap();
                        }
                        "open_after_record" => {
                            open_after_record.set(!capture_mouse_pointer.get());
                            window_.menu_handle().get_item(event.menu_item_id()).set_selected(open_after_record.get()).unwrap();
                        }
                        _ => {}
                    }
                    window_.emit_all(event.menu_item_id(), {}).unwrap();
                });

            Ok(())
        })
        .system_tray(SystemTray::new())
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
        .invoke_handler(tauri::generate_handler![capture, folder_dialog, current_default_path, log_message, write_to_json, get_home_dir, load_hotkeys])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn log_message(args: CmdArgs) {
    println!("{}", args.message);
}

#[tauri::command]
fn write_to_json(input: HotkeyInput) -> Result<(), String> {
    let path = get_home_dir() + "/.screencrab/hotkeys.json";
    let file_path = Path::new(&path);
    fs::write(file_path, input.hotkeyData.to_string()).map_err(|e| e.to_string())

}

