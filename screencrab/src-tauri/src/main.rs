use chrono::prelude::*;
use tauri::{Window, AppHandle, PhysicalSize, PhysicalPosition, App};
use std::path::Path;
use crate::menu::{create_context_menu, Hotkeys};
use tauri::{Manager, SystemTray, SystemTrayEvent};
mod menu;


use std::fs;
use serde_json;

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

pub fn load_hotkeys(filename: &str) -> Result<Hotkeys, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(filename)?;
    let hotkeys = serde_json::from_str(&contents)?;
    Ok(hotkeys)
}

fn open_new_window(app: tauri::AppHandle) {
    let new_window = tauri::WindowBuilder::new(
        &app,
        "new_window",
        tauri::WindowUrl::App("hotkeys_menu.html".into()))
        .title("Change Hotkeys")
        // Configura altre opzioni per la nuova finestra qui
        .build()
        .unwrap();

    new_window.show().unwrap();
}



fn main() {

    let hotkeys = load_hotkeys("src/hotkeys.json").expect("Failed to read the file");

    tauri::Builder::default()
        .menu(create_context_menu(&hotkeys))
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "change_hotkeys" => {
                    open_new_window(event.window().app_handle().clone());
                },
                _ => {
                    event.window().emit_all(event.menu_item_id(), {}).unwrap();
                }
            }
        })        
        .setup(|app| {
            
            let monitor_size = *app.get_window("main").unwrap().current_monitor().unwrap().unwrap().size();
            let width = monitor_size.width*70/100;
            let height = monitor_size.height*26/100;
            app.handle().windows().get("main").unwrap().set_size(PhysicalSize::new(width, height)).unwrap();
            app.handle().windows().get("main").unwrap().set_position(PhysicalPosition::new((monitor_size.width-width)/2, monitor_size.height-height*16/10)).unwrap();
            app.handle().windows().get("main").unwrap().set_resizable(false).unwrap();
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
                .build().unwrap();

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
                .build().unwrap();

            area.set_size(PhysicalSize::new(width/2, height)).unwrap();
            area.hide().unwrap();
                

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
        .invoke_handler(tauri::generate_handler![capture, folder_dialog, current_default_path, log_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[tauri::command]
fn log_message(args: CmdArgs) {
    println!("{}", args.message);
}