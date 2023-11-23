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
fn get_image_bytes(path: String) -> Vec<u8> {
    // Read the image file at runtime and return its bytes
    let image_bytes = std::fs::read(path).expect("Failed to read image file");
    image_bytes
}

#[tauri::command]
fn log_message(args: CmdArgs) {
    println!("{}", args.message);
}

#[tauri::command]
fn custom_area_selection(app: AppHandle, id: String, left: f64, top: f64, width: f64, height: f64) {
    let offset = LogicalPosition::new(app.windows().get(id.as_str()).unwrap().outer_position().unwrap().x as f64,app.windows().get(id.as_str()).unwrap().outer_position().unwrap().y as f64);
    let scale_factor = app.windows().get(id.as_str()).unwrap().current_monitor().unwrap().unwrap().scale_factor();
    let position = LogicalPosition::new(left + offset.x/scale_factor, top + offset.y/scale_factor);
    let size = LogicalSize::new(width, height);

    let n = app.windows().get("main_window").unwrap().available_monitors().unwrap().len();
    for i in 0..n {
        app.windows().get(format!("helper_{}", i).as_str()).unwrap().hide().unwrap();
    }

    app.windows().get("selector").unwrap().set_size(size).unwrap();
    app.windows().get("selector").unwrap().set_position(position).unwrap();
    app.windows().get("selector").unwrap().show().unwrap();
    app.windows().get("main_window").unwrap().set_focus().unwrap();

}

#[tauri::command]
fn show_all_helpers(app: AppHandle) {
    app.windows().get("selector").unwrap().hide().unwrap();
    let monitors = app.windows().get("main_window").unwrap().available_monitors().unwrap();
    for (i, monitor) in monitors.iter().enumerate() {
        app.windows().get(format!("helper_{}", i).as_str()).unwrap().set_position(monitor.position().to_logical::<f64>(monitor.scale_factor())).unwrap();
        app.windows().get(format!("helper_{}", i).as_str()).unwrap().set_size(monitor.size().to_logical::<f64>(monitor.scale_factor())).unwrap();
        app.windows().get(format!("helper_{}", i).as_str()).unwrap().show().unwrap();
    }
}

#[tauri::command]
fn hide_all_helpers(app: AppHandle) {
    app.windows().get("selector").unwrap().hide().unwrap();
    let monitors = app.windows().get("main_window").unwrap().available_monitors().unwrap();
    for (i, monitor) in monitors.iter().enumerate() {
        app.windows().get(format!("helper_{}", i).as_str()).unwrap().hide().unwrap();
    }
}

#[tauri::command]
fn write_to_json(app: AppHandle, input: HotkeyInput) {
    let path = utils::utils_dir() + "/hotkeys.json";
    let file_path = Path::new(&path);
    fs::write(file_path, input.hotkey_data.to_string()).unwrap();
    process::restart(&app.env())
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

fn main() {
    tauri::Builder::default()
        .setup(|app| {

            //Extract information about current monitor by the start_window defined in tauri.conf.json
            let monitor = app.windows().get("start_window").unwrap().primary_monitor().unwrap().unwrap();
            let scale_factor = app.windows().get("start_window").unwrap().scale_factor().unwrap();
            let monitor_size = monitor.size();

            let tools = tauri::WindowBuilder::new(
                app,
                "tools",
                tauri::WindowUrl::App("./tools.html".into()))
                .decorations(true)
                .visible(false)
                .inner_size((monitor_size.width as f64) * 0.9f64/scale_factor, (monitor_size.height as f64) * 0.8f64/scale_factor )
                .position((monitor_size.width as f64) * 0.05f64/scale_factor, (monitor_size.height as f64) * 0.1f64/scale_factor)
                .resizable(true)
                .closable(true)
                .always_on_top(true)
                .title("ScreenCrab Tools")
                .minimizable(true)
                .maximizable(true)
                .focused(true)
                .build()
                .unwrap();

            let hotkeys = tauri::WindowBuilder::new(
                app,
                "hotkeys",
                tauri::WindowUrl::App("./hotkeys.html".into()))
                .decorations(true)
                .visible(false)
                .inner_size((monitor_size.width as f64) * 0.6f64/scale_factor, (monitor_size.height as f64) * 0.9f64/scale_factor )
                .position((monitor_size.width as f64) * 0.2f64/scale_factor, (monitor_size.height as f64) * 0.05f64/scale_factor)
                .resizable(true)
                .closable(false)
                .always_on_top(true)
                .title("Shortcut Keys")
                .minimizable(true)
                .focused(true)
                .build()
                .unwrap();

            #[cfg(target_os = "macos")]
                let area = tauri::WindowBuilder::new(
                app,
                "selector",
                tauri::WindowUrl::App("./blank.html".into()))
                .menu(create_context_menu())
                .title_bar_style(TitleBarStyle::Overlay)
                .decorations(false)
                .visible(false)
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
                .visible(false)
                .transparent(true)
                .always_on_top(false)
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
                .always_on_top(false)
                .skip_taskbar(true)
                .center()
                .title("")
                .minimizable(false)
                .focused(false)
                .build()
                .unwrap();

            let available_monitors = area.available_monitors().unwrap();
            let mut helpers = Vec::with_capacity(available_monitors.len());
            for (i,monitor) in available_monitors.iter().enumerate() {
                let monitor_size = monitor.size().to_owned();
                #[cfg(target_os = "macos")] {
                    helpers.push(tauri::WindowBuilder::new(
                        app,
                        format!("helper_{}", i),
                        tauri::WindowUrl::App("./helper.html".into()))
                        .title_bar_style(TitleBarStyle::Overlay)
                        .menu(create_context_menu())
                        .visible(false)
                        .title("Select an area to capture...")
                        .decorations(false)
                        .transparent(true)
                        .resizable(false)
                        .always_on_top(true)
                        .minimizable(false)
                        .maximized(true)
                        .focused(true)
                        .build()
                        .unwrap()
                    );
                }

                #[cfg(target_os = "windows")] {
                    helpers.push(tauri::WindowBuilder::new(
                        app,
                        format!("helper_{}", i),
                        tauri::WindowUrl::App("./helper.html".into()))
                        .decorations(false)
                        .visible(false)
                        .title("Select an area to capture...")
                        .transparent(true)
                        .resizable(false)
                        .always_on_top(true)
                        .minimizable(false)
                        .focused(true)
                        .build()
                        .unwrap()
                    );
                }

                #[cfg(target_os = "linux")] {
                    helpers.push(tauri::WindowBuilder::new(
                        app,
                        format!("helper_{}", i),
                        tauri::WindowUrl::App("./helper.html".into()))
                        .decorations(false)
                        .visible(false)
                        .title("Select an area to capture...")
                        .transparent(true)
                        .resizable(false)
                        .always_on_top(true)
                        .minimizable(false)
                        .focused(true)
                        .build()
                        .unwrap()
                    );
                }

                helpers[i].set_position(monitor.position().to_logical::<f64>(monitor.scale_factor())).unwrap();
                helpers[i].set_size(monitor.size().to_logical::<f64>(monitor.scale_factor())).unwrap();
            }

            let main_window = tauri::WindowBuilder::new(
                app,
                "main_window",
                tauri::WindowUrl::App("./index.html".into()))
                .menu(create_context_menu())
                .visible(true)
                .fullscreen(false)
                .inner_size((monitor_size.width as f64) * 0.6f64/scale_factor, (monitor_size.height as f64) * 0.23f64/scale_factor )
                .position((monitor_size.width as f64) * 0.2f64/scale_factor, (monitor_size.height as f64) * 0.67f64/scale_factor)
                .resizable(true)
                .closable(true)
                .always_on_top(true)
                .minimizable(true)
                .focused(true)
                .title("Screen Crab")
                .content_protected(true)
                .decorations(true)
                .build()
                .unwrap();

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
            let edit_after_capture = Arc::new(Mutex::new(true));
            let record_external_audio = Arc::new(Mutex::new(false));
            let open_after_record = Arc::new(Mutex::new(true));
            let hotkeys_ = hotkeys.clone();

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
                    "change_hotkeys" => {
                        hotkeys_.show().unwrap();
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
                let hotkeys_ = hotkeys.clone();

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
                        "change_hotkeys" => {
                            hotkeys_.show().unwrap();
                        }
                        _ => {}
                    }
                    area_.emit_to("main_window", event.menu_item_id(), {}).unwrap();
                });


                for helper in helpers {
                    let helper_ = helper.clone();
                    let capture_mouse_pointer_ = capture_mouse_pointer.clone();
                    let copy_to_clipboard_ = copy_to_clipboard.clone();
                    let edit_after_capture_ = edit_after_capture.clone();
                    let record_external_audio_ = record_external_audio.clone();
                    let open_after_record_ = open_after_record.clone();
                    let hotkeys_ = hotkeys.clone();
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
                            "change_hotkeys" => {
                                hotkeys_.show().unwrap();
                            }
                            _ => {}
                        }
                        helper_.emit_to("main_window", event.menu_item_id(), {}).unwrap();
                    });
                }
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
        .invoke_handler(tauri::generate_handler![capture, get_image_bytes, folder_dialog, current_default_path, log_message, write_to_json, load_hotkeys, close_hotkeys, window_hotkeys, custom_area_selection, show_all_helpers, hide_all_helpers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
