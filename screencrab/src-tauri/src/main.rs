#![cfg_attr(all(not(debug_assertions), target_os = "macos"), windows_subsystem = "console")]
#![cfg_attr(all(not(debug_assertions), target_os = "windows"),windows_subsystem = "windows")]
mod menu;
mod utils;

use chrono::prelude::*;
use tauri::{Window, AppHandle, WindowEvent};
use std::path::Path;
use crate::menu::create_context_menu;
use crate::utils::Response;
use tauri::{Manager, SystemTray, SystemTrayEvent};
use std::sync::Arc;
use std::sync::Mutex;
use tauri::api::notification::Notification;

#[cfg(target_os="macos")]
use tauri::utils::TitleBarStyle;
#[cfg(target_os = "macos")]
use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
#[cfg(target_os = "macos")]
use cocoa::appkit::NSWindowTitleVisibility;
#[cfg(target_os = "macos")]
use cocoa::appkit::NSWindowStyleMask;


#[cfg(target_os = "macos")]
mod darwin;


#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use serde_json::Value;
#[cfg(target_os = "windows")]
use winapi_easy::keyboard::GlobalHotkeySet;

#[cfg(target_os = "linux")]
mod linux;

#[tauri::command(rename_all = "snake_case")]
async fn capture(app: AppHandle, window: Window, mode: &str, view: &str, area: &str, timer: u64, pointer: bool, file_path: &str, file_type: &str, clipboard: bool, audio: bool, open_file: bool) -> Result<Response, String> {
    let abs_path: String;
    let fs_path = Path::new(file_path);

    app.windows().get("main_window").unwrap().set_content_protected(true).unwrap();

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

    app.windows().get("main_window").unwrap().set_content_protected(false).unwrap();
    return Ok(result);
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {

            //Extract information about current monitor by the start_window defined in tauri.conf.json
            let monitor = app.windows().get("start_window").unwrap().primary_monitor().unwrap().unwrap();
            let scale_factor = app.windows().get("start_window").unwrap().scale_factor().unwrap();
            let monitor_size = monitor.size();

            let hotkeys = tauri::WindowBuilder::new(
                app,
                "hotkeys",
                tauri::WindowUrl::App("./hotkeys.html".into()))
                .decorations(true)
                .visible(false)
                .inner_size((monitor_size.width as f64) * 0.6f64/scale_factor, (monitor_size.height as f64) * 0.9f64/scale_factor )
                .position((monitor_size.width as f64) * 0.2f64/scale_factor, (monitor_size.height as f64) * 0.05f64/scale_factor)
                .resizable(true)
                .closable(true)
                .always_on_top(true)
                .title("Shortcut Keys")
                .minimizable(true)
                .focused(true)
                .build()
                .unwrap();

            let hotkeys_ = hotkeys.clone();
            hotkeys.on_window_event(move |event| {
                match event {
                    WindowEvent::CloseRequested{api, ..} => {
                        api.prevent_close();
                        hotkeys_.hide().unwrap(); }
                    _ => {}
                }
            });

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
                .content_protected(false)
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
                .resizable(true)
                .always_on_top(true)
                .skip_taskbar(true)
                .center()
                .title("")
                .content_protected(false)
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
                .visible(false)
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

            let available_monitors = area.available_monitors().unwrap();
            let mut helpers = Vec::with_capacity(available_monitors.len());
            for (i,monitor) in available_monitors.iter().enumerate() {
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

            let main_window;

            #[cfg(target_os = "linux")] {
                main_window = tauri::WindowBuilder::new(
                    app,
                    "main_window",
                    tauri::WindowUrl::App("./index.html".into()))
                    .menu(create_context_menu())
                    .visible(true)
                    .fullscreen(false)
                    //adjust size & position for the platform
                    .inner_size((monitor_size.width as f64) * 0.65f64 / scale_factor, (monitor_size.height as f64) * 0.28f64 / scale_factor)
                    .position((monitor_size.width as f64) * 0.2f64 / scale_factor, (monitor_size.height as f64) * 0.67f64 / scale_factor)
                    .resizable(true)
                    .closable(true)
                    .always_on_top(true)
                    .minimizable(true)
                    .focused(true)
                    .title("Screen Crab")
                    .content_protected(false)
                    .decorations(true)
                    .build()
                    .unwrap();
            }

            #[cfg(target_os = "windows")] {
                main_window = tauri::WindowBuilder::new(
                    app,
                    "main_window",
                    tauri::WindowUrl::App("./index.html".into()))
                    .menu(create_context_menu())
                    .visible(true)
                    .fullscreen(false)
                    //adjust size & position for the platform
                    .inner_size((monitor_size.width as f64) * 0.63f64 / scale_factor, (monitor_size.height as f64) * 0.25f64 / scale_factor)
                    .position((monitor_size.width as f64) * 0.2f64 / scale_factor, (monitor_size.height as f64) * 0.67f64 / scale_factor)
                    .resizable(true)
                    .closable(true)
                    .always_on_top(true)
                    .minimizable(true)
                    .focused(true)
                    .title("Screen Crab")
                    .content_protected(false)
                    .decorations(true)
                    .build()
                    .unwrap();
            }

            #[cfg(target_os = "macos")] {
                main_window = tauri::WindowBuilder::new(
                    app,
                    "main_window",
                    tauri::WindowUrl::App("./index.html".into()))
                    .menu(create_context_menu())
                    .visible(true)
                    .fullscreen(false)
                    //adjust size & position for the platform
                    .inner_size((monitor_size.width as f64) * 0.6f64/scale_factor, (monitor_size.height as f64) * 0.24f64/scale_factor )
                    .position((monitor_size.width as f64) * 0.2f64/scale_factor, (monitor_size.height as f64) * 0.67f64/scale_factor)
                    .resizable(true)
                    .closable(true)
                    .always_on_top(true)
                    .minimizable(true)
                    .focused(true)
                    .title("Screen Crab")
                    .title_bar_style(TitleBarStyle::Transparent)
                    .content_protected(false)
                    .decorations(true)
                    .build()
                    .unwrap();
                unsafe {
                    let id = main_window.ns_window().unwrap() as cocoa::base::id;
                    #[cfg(target_arch = "x86_64")]
                    NSWindow::setMovableByWindowBackground_(id, 1);
                    #[cfg(target_arch = "aarch64")]
                    NSWindow::setMovableByWindowBackground_(id, true);
                    NSWindow::setCollectionBehavior_(id, NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces);
                    NSWindow::setCollectionBehavior_(id, NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace);
                    NSWindow::setCollectionBehavior_(id, NSWindowCollectionBehavior::NSWindowCollectionBehaviorTransient);
                    #[cfg(target_arch = "x86_64")]
                    NSWindow::setTitlebarAppearsTransparent_(id, 1);
                    #[cfg(target_arch = "aarch64")]
                    NSWindow::setTitlebarAppearsTransparent_(id, true);
                    let mut style_mask = id.styleMask();
                    style_mask.set(
                        NSWindowStyleMask::NSFullSizeContentViewWindowMask,
                        true,
                    );
                    id.setStyleMask_(style_mask);
                    NSWindow::setTitleVisibility_(id, NSWindowTitleVisibility::NSWindowTitleHidden);
                    #[cfg(target_arch = "x86_64")]
                    NSWindow::setTitlebarAppearsTransparent_(id, 1);
                    #[cfg(target_arch = "aarch64")]
                    NSWindow::setTitlebarAppearsTransparent_(id, true);
                }
            }

            let main_window_ = main_window.clone();
            main_window.on_window_event(move |event| {
                match event {
                    WindowEvent::CloseRequested{..} => {
                        main_window_.app_handle().exit(0);
                    }
                    _ => {}
                }
            });

            let main_window__ = main_window.clone();
            area.on_window_event(move |event| {
                match event {
                    WindowEvent::Resized(_) | WindowEvent::Moved(_) => {
                        main_window__.set_focus().unwrap();
                    }
                    _ => {}
                }
            });

            let capture_mouse_pointer = Arc::new(Mutex::new(false));
            let copy_to_clipboard = Arc::new(Mutex::new(false));
            let edit_after_capture = Arc::new(Mutex::new(true));
            let record_external_audio = Arc::new(Mutex::new(false));
            let open_after_record = Arc::new(Mutex::new(true));
            let hotkeys_ = hotkeys.clone();

            #[cfg(target_os = "windows")] {
                let capture_mouse_pointer_ = capture_mouse_pointer.clone();
                let copy_to_clipboard_ = copy_to_clipboard.clone();
                let edit_after_capture_ = edit_after_capture.clone();
                let record_external_audio_ = record_external_audio.clone();
                let open_after_record_ = open_after_record.clone();

                let hotkeys_string = utils::hotkeys();
                let hotkeys_dict: Value = serde_json::from_str(hotkeys_string.as_str()).unwrap();

                let map = utils::create_mapping(hotkeys_dict);
    
                let main_window_ = main_window.clone();
                let _task = tokio::task::spawn( async move {
                    let hotkeys = GlobalHotkeySet::new()
                        .add_global_hotkey("fullscreen_capture", map.get("fullscreen_capture").unwrap().clone())
                        .add_global_hotkey("custom_capture", map.get("custom_capture").unwrap().clone())
                        .add_global_hotkey("copy_to_clipboard", map.get("copy_to_clipboard").unwrap().clone())
                        .add_global_hotkey("capture_mouse_pointer", map.get("capture_mouse_pointer").unwrap().clone())
                        .add_global_hotkey("custom_record", map.get("custom_record").unwrap().clone())
                        .add_global_hotkey("edit_after_capture", map.get("edit_after_capture").unwrap().clone())
                        .add_global_hotkey("fullscreen_record", map.get("fullscreen_record").unwrap().clone())
                        .add_global_hotkey("open_after_record", map.get("open_after_record").unwrap().clone())
                        .add_global_hotkey("record_external_audio", map.get("record_external_audio").unwrap().clone())
                        .add_global_hotkey("stop_recording", map.get("stop_recording").unwrap().clone());
                        match hotkeys.listen_for_hotkeys() {
                            Ok(iterator) => {
                                // Now iterate over the iterator if Result is Ok
                                for action_result in iterator {
                                    let action = action_result.unwrap();
                                    match action {
                                            "copy_to_clipboard" => {
                                                let mut data = copy_to_clipboard_.lock().unwrap();
                                                *data = !*data;
                                                let mut value = edit_after_capture_.lock().unwrap();
                                                *value = !*data;
                                                main_window_.menu_handle().get_item("copy_to_clipboard").set_selected(*data).unwrap();
                                                main_window_.menu_handle().get_item("edit_after_capture").set_enabled(!*data).unwrap();
                                            }
                                            "capture_mouse_pointer" => {
                                                let mut data = capture_mouse_pointer_.lock().unwrap();
                                                *data = !*data;
                                                main_window_.menu_handle().get_item("capture_mouse_pointer").set_selected(*data).unwrap();
                                            }
                                            "edit_after_capture" => {
                                                let mut data = edit_after_capture_.lock().unwrap();
                                                *data = !*data;
                                                main_window_.menu_handle().get_item("edit_after_capture").set_selected(*data).unwrap();
                                            }
                                            "open_after_record" => {
                                                let mut data = open_after_record_.lock().unwrap();
                                                *data = !*data;
                                                main_window_.menu_handle().get_item("open_after_record").set_selected(*data).unwrap();
                                            }
                                            "record_external_audio" => {
                                                let mut data = record_external_audio_.lock().unwrap();
                                                *data = !*data;
                                                main_window_.menu_handle().get_item("record_external_audio").set_selected(*data).unwrap();
                                            }
                                            _ => {}
                                        }
                                        main_window_.emit(action, {}).unwrap();
                                    }
                                }
                            Err(e) => eprintln!("Error listening for hotkeys: {:?}", e),
                        }
                    });
                }   

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
                    "learn_more" => {
                        webbrowser::open("https://github.com/grzianod/screencrab/tree/main/screencrab#screen-crab").unwrap();
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
                        "learn_more" => {
                            webbrowser::open("https://github.com/grzianod/screencrab/blob/main/screencrab/README.md").unwrap();
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
                            "learn_more" => {
                                webbrowser::open("https://github.com/grzianod/screencrab/blob/main/screencrab/README.md").unwrap();
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
        .invoke_handler(tauri::generate_handler![capture, utils::folder_dialog, utils::current_default_path, utils::write_to_json, utils::load_hotkeys, utils::close_hotkeys, utils::window_hotkeys, utils::custom_area_selection, utils::show_all_helpers, utils::hide_all_helpers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
