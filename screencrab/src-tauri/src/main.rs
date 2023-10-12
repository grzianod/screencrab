#![cfg_attr(all(not(debug_assertions), target_os = "macos"), windows_subsystem = "console")]

mod menu;
mod utils;

use chrono::prelude::*;
use tauri::{Window, AppHandle, PhysicalSize, PhysicalPosition, Icon, CursorIcon};
use std::path::Path;
use crate::menu::{create_context_menu};
use crate::utils::{Response, utils_dir};
use tauri::{Manager, SystemTray, SystemTrayEvent, api::process};
use std::sync::Arc;
use std::sync::Mutex;
use tauri::api::notification::Notification;
use std::{env, fs};
use serde_json;
use tauri::{LogicalPosition, LogicalSize};


#[derive(serde::Deserialize)]
struct HotkeyInput {
    hotkey_data: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct CmdArgs {
    message: String,
}

#[cfg(target_os="macos")]
use tauri::TitleBarStyle;
use tauri::utils::TitleBarStyle;
#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior, NSCursor};
#[cfg(target_os = "macos")]
use cocoa::appkit::NSWindowTitleVisibility;
#[cfg(target_os = "macos")]
use cocoa::appkit::NSWindowStyleMask;
use tauri_plugin_positioner::WindowExt;


#[cfg(target_os = "macos")]
mod darwin;


#[cfg(target_os = "windows")]
mod windows;


#[cfg(target_os = "linux")]
mod linux;

#[tauri::command]
async fn folder_dialog(handle: AppHandle) -> Response {
    return utils::folder_picker(handle).await;
}

#[tauri::command]
async fn current_default_path() -> Response {
    return utils::current_default_path().await;
}

#[tauri::command]
fn log_message(args: CmdArgs) {
    println!("{}", args.message);
}

#[tauri::command]
fn custom_area_selection(app: AppHandle, x: f64, y: f64, width: f64, height: f64) {
    let offset = LogicalPosition::new(app.windows().get("helper").unwrap().inner_position().unwrap().x as f64,app.windows().get("helper").unwrap().outer_position().unwrap().y as f64);
    let pos = LogicalPosition::new(x + offset.x/2f64, y + offset.y/2f64);

    app.windows().get("helper").unwrap().hide().unwrap();
    app.windows().get("selector").unwrap().set_size(LogicalSize::new(width, height)).unwrap();
    app.windows().get("selector").unwrap().set_position(pos).unwrap();
    app.windows().get("selector").unwrap().show().unwrap();
    app.windows().get("helper").unwrap().minimize().unwrap();
}

#[tauri::command]
fn write_to_json(app: AppHandle, input: HotkeyInput) {
    let path = get_home_dir() + "/.screencrab/hotkeys.json";
    let file_path = Path::new(&path);
    fs::write(file_path, input.hotkey_data.to_string()).unwrap();
    process::restart(&app.env())
}

#[tauri::command(rename_all = "snake_case")]
fn check_requirements(app: AppHandle) -> Result<(), String> {
    #[cfg(target_os = "macos")] {
        app.windows().get("splashscreen").unwrap().hide().unwrap();
        app.windows().get("main_window").unwrap().show().unwrap();
        fs::write(utils_dir() + "/marker.json", b"1").unwrap();
    }
    #[cfg(target_os = "windows")] {
        app.windows().get("splashscreen").unwrap().hide().unwrap();
        app.windows().get("main_window").unwrap().show().unwrap();
        fs::write(utils_dir() + "/marker.json", b"1").unwrap();
        //TODO: ffmpeg check or installation
    }
    #[cfg(target_os = "linux")] {
        //TODO: ffmpeg check or installation
    }
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
async fn capture(app: AppHandle, window: Window, mode: &str, view: &str, area: &str, timer: u64, pointer: bool, file_path: &str, file_type: &str, clipboard: bool, audio: bool, open_file: bool) -> Result<Response, String> {
    let abs_path: String;
    let fs_path = Path::new(file_path);

    if (!cfg!(target_os="windows") && file_path.ends_with("/") && (!fs_path.exists() || !fs_path.is_dir())) || ((cfg!(target_os="windows") && file_path.ends_with("\\") && (!fs_path.exists() || !fs_path.is_dir()))) {
        return Err(format!("\"{}\" is not a valid directory.", file_path));
    }

    if (!cfg!(target_os="windows") && file_path.ends_with("/")) || (cfg!(target_os="windows") && file_path.ends_with("\\")) {
        let current_date = Local::now();
        let formatted_date = current_date.format("%Y-%m-%d at %H-%M-%S").to_string();
        abs_path = format!("{}Screen Crab {}.{}", file_path, formatted_date, file_type);
    } else {
        abs_path = format!("{}.{}", file_path, file_type);
    }

    let result: Response;

    match mode {
        "capture" => {
            match view {
                "fullscreen" => {
                    #[cfg(target_os = "macos")] {
                        result = darwin::capture_fullscreen(window, abs_path.as_str(), &file_type, timer, pointer, clipboard, audio, open_file).await;
                    }
                    #[cfg(target_os = "linux")] {
                        result = linux::capture_fullscreen(window, abs_path.as_str(), &file_type, timer, pointer, clipboard, audio, open_file).await;
                    }
                    #[cfg(target_os = "windows")] {
                        result = windows::capture_fullscreen(window, abs_path.as_str(), &file_type, timer, pointer, clipboard, audio, open_file).await;
                    }
                }
                "custom" => {
                    #[cfg(target_os = "macos")] {
                        result = darwin::capture_custom(window, area, abs_path.as_str(), &file_type, timer, pointer, clipboard, audio, open_file).await;
                    }
                    #[cfg(target_os = "linux")] {
                        result = linux::capture_custom(window, area, abs_path.as_str(), &file_type, timer, pointer, clipboard, audio, open_file).await;
                    }
                    #[cfg(target_os = "windows")] {
                        result = windows::capture_custom(window, area, abs_path.as_str(), &file_type, timer, pointer, clipboard, audio, open_file).await;
                    }
                }
                _ => return Ok(Response::new(None, Some(format!("Invalid view: {}", view))))
            }
        }
        "record" => {
            match view {
                "fullscreen" => {
                    #[cfg(target_os = "macos")] {
                        result = darwin::record_fullscreen(window, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await;
                    }
                    #[cfg(target_os = "linux")] {
                        result = linux::record_fullscreen(window, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await;
                    }
                    #[cfg(target_os = "windows")] {
                        result = windows::record_fullscreen(window, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await;
                    }
                }
                "custom" => {
                    #[cfg(target_os = "macos")] {
                        result = darwin::record_custom(window, area, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await;
                    }
                    #[cfg(target_os = "linux")] {
                        result = linux::record_custom(window, area, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await;
                    }
                    #[cfg(target_os = "windows")] {
                        result = windows::record_custom(window, area, abs_path.as_str(), timer, pointer, clipboard, audio, open_file).await;
                    }
                }
                _ => return Ok(Response::new(None, Some(format!("Invalid view: {}", view))))
            }
        }
        _ => return Ok(Response::new(None, Some(format!("Invalid mode: {}", mode))))
    }

    if result.success() {
        Notification::new(&app.config().tauri.bundle.identifier)
            .title("All done!")
            .body(format!("{}", result.response().unwrap()))
            .icon("icons/icon.icns").show().unwrap();
    } else {
        Notification::new(&app.config().tauri.bundle.identifier)
            .body(format!("{}", result.error().unwrap()))
            .icon("icons/icon.icns").show().unwrap();
    }

    return Ok(result);
}

#[tauri::command]
fn get_home_dir() -> String {
    env::var("HOME").unwrap()
}

#[tauri::command]
async fn load_hotkeys() -> String {
    utils::hotkeys()
}

#[tauri::command]
fn window_hotkeys(app: AppHandle) {
    app.windows().get("hotkeys").unwrap().show().unwrap();
}

#[tauri::command]
fn close_hotkeys(app: AppHandle) {
    app.windows().get("hotkeys").unwrap().hide().unwrap();
}

fn splashscreen() -> bool {
    let path = utils_dir() + "/marker.json";
    match fs::metadata(path.clone()) {
        Ok(_) => { false }
        Err(_) => { true }
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let hotkeys = tauri::WindowBuilder::new(
                app,
                "hotkeys",
                tauri::WindowUrl::App("./hotkeys.html".into()))
                .decorations(true)
                .resizable(false)
                .closable(false)
                .always_on_top(true)
                .title("Shortcut Keys")
                .minimizable(false)
                .focused(true)
                .build()
                .unwrap();
            hotkeys.set_size(PhysicalSize::new(1600, 1500)).unwrap();
            hotkeys.hide().unwrap();

            #[cfg(target_os = "macos")]
                let area = tauri::WindowBuilder::new(
                app,
                "selector",
                tauri::WindowUrl::App("./blank.html".into()))
                .menu(create_context_menu())
                .title_bar_style(TitleBarStyle::Overlay)
                .decorations(false)
                .transparent(true)
                .resizable(true)
                .skip_taskbar(true)
                .center()
                .title("")
                .always_on_top(true)
                .content_protected(true)
                .minimizable(false)
                .focused(false)
                .build()
                .unwrap();


            #[cfg(target_os = "linux")]
                let area = tauri::WindowBuilder::new(
                app,
                "selector",
                tauri::WindowUrl::App("./blank.html".into()))
                .decorations(false)
                .transparent(true)
                .always_on_top(true)
                .resizable(true)
                .always_on_top(true)
                .skip_taskbar(true)
                .center()
                .title("")
                .content_protected(true)
                .minimizable(false)
                .focused(false)
                .build()
                .unwrap();

            #[cfg(target_os = "windows")]
                let area = tauri::WindowBuilder::new(
                app,
                "selector",
                tauri::WindowUrl::App("./blank.html".into()))
                .decorations(false)
                .transparent(true)
                .resizable(true)
                .always_on_top(true)
                .skip_taskbar(true)
                .center()
                .title("")
                .minimizable(false)
                .focused(false)
                .build()
                .unwrap();

            #[cfg(target_os = "macos")]
            let helper = tauri::WindowBuilder::new(
                app,
                "helper",
                tauri::WindowUrl::App("./helper.html".into()))
                .title_bar_style(TitleBar::Overlay)
                .menu(create_context_menu())
                .decorations(false)
                .transparent(true)
                .resizable(false)
                .always_on_top(true)
                .minimizable(false)
                .maximized(true)
                .focused(true)
                .build()
                .unwrap();

            #[cfg(target_os = "windows")]
            let helper = tauri::WindowBuilder::new(
                app,
                "helper",
                tauri::WindowUrl::App("./helper.html".into()))
                .decorations(false)
                .transparent(true)
                .resizable(false)
                .always_on_top(true)
                .minimizable(false)
                .focused(true)
                .build()
                .unwrap();

            #[cfg(target_os = "linux")]
                let helper = tauri::WindowBuilder::new(
                app,
                "helper",
                tauri::WindowUrl::App("./helper.html".into()))
                .decorations(false)
                .transparent(true)
                .resizable(false)
                .always_on_top(true)
                .minimizable(false)
                .focused(true)
                .build()
                .unwrap();

            helper.hide().unwrap();

            let monitor_size = area.current_monitor().unwrap().unwrap().size().to_owned();

            let width;
            let height;
            if cfg!(target_os="windows") {
                width = monitor_size.width * 65 / 100;
                height = monitor_size.height * 25 / 100;
            } else {
                width = monitor_size.width * 60 / 100;
                height = monitor_size.height * 23 / 100;
            }

            area.hide().unwrap();

            let main_window = tauri::WindowBuilder::new(
                app,
                "main_window",
                tauri::WindowUrl::App("./index.html".into()))
                .menu(create_context_menu())
                .visible(false)
                .fullscreen(false)
                .resizable(false)
                .closable(true)
                .always_on_top(false)
                .minimizable(false)
                .focused(true)
                .title("Screen Crab")
                .content_protected(true)
                .decorations(true)
                .build()
                .unwrap();

            main_window.set_size(PhysicalSize::new(width, height)).unwrap();
            main_window.set_position(PhysicalPosition::new((monitor_size.width - width) / 2, monitor_size.height - height * 16 / 10)).unwrap();

            if splashscreen() {
                let splash = tauri::WindowBuilder::new(
                    app,
                    "splashscreen",
                    tauri::WindowUrl::App("./splashscreen.html".into()))
                    .decorations(true)
                    .resizable(false)
                    .always_on_top(false)
                    .title("Screen Crab")
                    .minimizable(true)
                    .focused(true)
                    .build()
                    .unwrap();
                splash.set_size(PhysicalSize::new(1100, 1500)).unwrap();
                splash.show().unwrap();
                main_window.hide().unwrap();
            } else {
                main_window.show().unwrap();
            }

            #[cfg(target_os = "macos")]
            unsafe {
                let id = main_window.ns_window().unwrap() as cocoa::base::id;
                NSWindow::setCollectionBehavior_(id, NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces);
                NSWindow::setCollectionBehavior_(id, NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace);
                NSWindow::setCollectionBehavior_(id, NSWindowCollectionBehavior::NSWindowCollectionBehaviorTransient);
                NSWindow::setMovableByWindowBackground_(id, 1);
                let mut style_mask = id.styleMask();
                style_mask.set(
                    NSWindowStyleMask::NSFullSizeContentViewWindowMask,
                    true,
                );
                id.setStyleMask_(style_mask);
                NSWindow::setTitleVisibility_(id, NSWindowTitleVisibility::NSWindowTitleHidden);
                NSWindow::setTitlebarAppearsTransparent_(id, 1);
            }

            let capture_mouse_pointer = Arc::new(Mutex::new(false));
            let copy_to_clipboard = Arc::new(Mutex::new(false));
            let edit_after_capture = Arc::new(Mutex::new(false));
            let record_external_audio = Arc::new(Mutex::new(false));
            let open_after_record = Arc::new(Mutex::new(false));


            let window_ = main_window.clone();
            let capture_mouse_pointer_ = capture_mouse_pointer.clone();
            let copy_to_clipboard_ = copy_to_clipboard.clone();
            let edit_after_capture_ = edit_after_capture.clone();
            let record_external_audio_ = record_external_audio.clone();
            let open_after_record_ = open_after_record.clone();

            main_window.on_menu_event(move |event| {
                match event.menu_item_id() {
                    "capture_mouse_pointer" => {
                        let mut data = capture_mouse_pointer_.lock().unwrap();
                        *data = !*data;
                        window_.menu_handle().get_item(event.menu_item_id()).set_selected(*data).unwrap();
                    }
                    "copy_to_clipboard" => {
                        let mut data = copy_to_clipboard_.lock().unwrap();
                        *data = !*data;
                        let mut value = edit_after_capture_.lock().unwrap();
                        *value = !*data;
                        window_.menu_handle().get_item("copy_to_clipboard").set_selected(*data).unwrap();
                        window_.menu_handle().get_item("edit_after_capture").set_enabled(!*data).unwrap();
                    }
                    "edit_after_capture" => {
                        let mut data = edit_after_capture_.lock().unwrap();
                        *data = !*data;
                        window_.menu_handle().get_item(event.menu_item_id()).set_selected(*data).unwrap();
                    }
                    "record_external_audio" => {
                        let mut data = record_external_audio_.lock().unwrap();
                        *data = !*data;
                        window_.menu_handle().get_item(event.menu_item_id()).set_selected(*data).unwrap();
                    }
                    "open_after_record" => {
                        let mut data = open_after_record_.lock().unwrap();
                        *data = !*data;
                        window_.menu_handle().get_item(event.menu_item_id()).set_selected(*data).unwrap();
                    }
                    _ => {}
                }
                window_.emit_to("main_window", event.menu_item_id(), {}).unwrap();
            });

            #[cfg(target_os = "macos")] {
                let area_ = area.clone();
                let capture_mouse_pointer_ = capture_mouse_pointer.clone();
                let copy_to_clipboard_ = copy_to_clipboard.clone();
                let edit_after_capture_ = edit_after_capture.clone();
                let record_external_audio_ = record_external_audio.clone();
                let open_after_record_ = open_after_record.clone();

                area.on_menu_event(move |event| {
                    match event.menu_item_id() {
                        "capture_mouse_pointer" => {
                            let mut data = capture_mouse_pointer_.lock().unwrap();
                            *data = !*data;
                            area_.windows().get("main_window").unwrap().menu_handle().get_item(event.menu_item_id()).set_selected(*data).unwrap();
                        }
                        "copy_to_clipboard" => {
                            let mut data = copy_to_clipboard_.lock().unwrap();
                            *data = !*data;
                            let mut value = edit_after_capture_.lock().unwrap();
                            *value = !*data;
                            area_.windows().get("main_window").unwrap().menu_handle().get_item("copy_to_clipboard").set_selected(*data).unwrap();
                            area_.windows().get("main_window").unwrap().menu_handle().get_item("edit_after_capture").set_enabled(!*data).unwrap();
                        }
                        "edit_after_capture" => {
                            let mut data = edit_after_capture_.lock().unwrap();
                            *data = !*data;
                            area_.windows().get("main_window").unwrap().menu_handle().get_item(event.menu_item_id()).set_selected(*data).unwrap();
                        }
                        "record_external_audio" => {
                            let mut data = record_external_audio_.lock().unwrap();
                            *data = !*data;
                            area_.windows().get("main_window").unwrap().menu_handle().get_item(event.menu_item_id()).set_selected(*data).unwrap();
                        }
                        "open_after_record" => {
                            let mut data = open_after_record_.lock().unwrap();
                            *data = !*data;
                            area_.windows().get("main_window").unwrap().menu_handle().get_item(event.menu_item_id()).set_selected(*data).unwrap();
                        }
                        _ => {}
                    }
                    area_.emit_to("main_window", event.menu_item_id(), {}).unwrap();
                });

                let helper_ = helper.clone();
                let capture_mouse_pointer_ = capture_mouse_pointer.clone();
                let copy_to_clipboard_ = copy_to_clipboard.clone();
                let edit_after_capture_ = edit_after_capture.clone();
                let record_external_audio_ = record_external_audio.clone();
                let open_after_record_ = open_after_record.clone();
                helper.on_menu_event(move |event| {
                    match event.menu_item_id() {
                        "capture_mouse_pointer" => {
                            let mut data = capture_mouse_pointer_.lock().unwrap();
                            *data = !*data;
                            helper_.windows().get("main_window").unwrap().menu_handle().get_item(event.menu_item_id()).set_selected(*data).unwrap();
                        }
                        "copy_to_clipboard" => {
                            let mut data = copy_to_clipboard_.lock().unwrap();
                            *data = !*data;
                            let mut value = edit_after_capture_.lock().unwrap();
                            *value = !*data;
                            helper_.windows().get("main_window").unwrap().menu_handle().get_item("copy_to_clipboard").set_selected(*data).unwrap();
                            helper_.windows().get("main_window").unwrap().menu_handle().get_item("edit_after_capture").set_enabled(!*data).unwrap();
                        }
                        "edit_after_capture" => {
                            let mut data = edit_after_capture_.lock().unwrap();
                            *data = !*data;
                            helper_.windows().get("main_window").unwrap().menu_handle().get_item(event.menu_item_id()).set_selected(*data).unwrap();
                        }
                        "record_external_audio" => {
                            let mut data = record_external_audio_.lock().unwrap();
                            *data = !*data;
                            helper_.windows().get("main_window").unwrap().menu_handle().get_item(event.menu_item_id()).set_selected(*data).unwrap();
                        }
                        "open_after_record" => {
                            let mut data = open_after_record_.lock().unwrap();
                            *data = !*data;
                            helper_.windows().get("main_window").unwrap().menu_handle().get_item(event.menu_item_id()).set_selected(*data).unwrap();
                        }
                        _ => {}
                    }
                    helper_.emit_to("main_window", event.menu_item_id(), {}).unwrap();
                });
            }

            Ok(())
        })
        .system_tray(SystemTray::new())
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main_window").unwrap();
                // toggle application window
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { .. } => {
                event.window().app_handle().exit(0);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![capture, folder_dialog, current_default_path, log_message, write_to_json, get_home_dir, load_hotkeys, close_hotkeys, window_hotkeys, check_requirements, custom_area_selection])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

