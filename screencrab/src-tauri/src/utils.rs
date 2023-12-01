use serde::{Serialize, Deserialize};
use tokio::sync::oneshot;
use tauri::api::dialog::FileDialogBuilder;

use tokio::task;
use tokio::process::Command;
use tauri::{AppHandle, Window, Manager};
use std::{env, fs};
use std::fs::File;
use std::io::Write;
use tauri::api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind};

use tauri::LogicalSize;
use tauri::LogicalPosition;
use tauri::api::process;


#[cfg(not(target_os = "macos"))]
use arboard::{Clipboard, ImageData, Error};
#[cfg(not(target_os = "macos"))]
use base64::{engine::general_purpose, Engine as _};
#[cfg(not(target_os = "macos"))]
use image::GenericImageView;
#[cfg(not(target_os = "macos"))]
use std::borrow::Cow;
use std::path::Path;

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    path: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    response: Option<String>,
    error: Option<String>,
}

impl Response {
    pub fn new(response: Option<String>, error: Option<String>) -> Self {
        Response { response, error }
    }

    pub fn success(&self) -> bool {
        self.response.is_some() && self.error.is_none()
    }
    pub fn failure(&self) -> bool {
        !self.success()
    }

    pub fn response(&self) -> Option<String> {
        if self.success() { self.response.clone() }
        else { None }
    }

    pub fn error(&self) -> Option<String> {
        if self.failure() { self.error.clone() }
        else { None }
    }

}

#[derive(Deserialize)]
pub struct Hotkeys {
    pub fullscreen_capture: String,
    pub custom_capture: String,
    pub capture_mouse_pointer: String,
    pub copy_to_clipboard: String,
    pub edit_after_capture: String,
    pub fullscreen_record: String,
    pub custom_record: String,
    pub stop_recording: String,
    pub record_external_audio: String,
    pub open_after_record : String
}

#[derive(serde::Deserialize)]
pub struct HotkeyInput {
    hotkey_data: serde_json::Value,
}

pub fn hotkeys() -> String {
    let file = utils_dir() + "/hotkeys.json";
    if let Ok(_result) = fs::create_dir(utils_dir()) {
        let json_content = r#"{
        "custom_capture": "CmdOrCtrl+C",
        "fullscreen_capture": "CmdOrCtrl+F",
        "capture_mouse_pointer": "Option+M",
        "copy_to_clipboard": "Option+C",
        "edit_after_capture": "Option+E",
        "open_after_record": "Option+O",
        "custom_record": "CmdOrCtrl+Option+C",
        "record_external_audio": "Option+A",
        "fullscreen_record": "CmdOrCtrl+Option+F",
        "stop_recording": "CmdOrCtrl+Option+S"
        }"#;

        // Create a new file and open it for writing
        let mut file = File::create(file.to_string()).unwrap();

        // Write the JSON content to the file
        file.write_all(json_content.as_bytes()).unwrap();
    }
    return fs::read_to_string(file).unwrap();
}
#[tauri::command]
pub fn write_to_json(app: AppHandle, input: HotkeyInput) {
    let path = utils_dir() + "/hotkeys.json";
    let file_path = Path::new(&path);
    fs::write(file_path, input.hotkey_data.to_string()).unwrap();
    process::restart(&app.env())
}

#[tauri::command]
pub async fn load_hotkeys() -> String {
    hotkeys()
}

#[tauri::command]
pub fn window_hotkeys(app: AppHandle) {
    app.windows().get("hotkeys").unwrap().show().unwrap();
}

#[tauri::command]
pub fn close_hotkeys(app: AppHandle) {
    app.windows().get("hotkeys").unwrap().hide().unwrap();
}

#[tauri::command]
pub async fn folder_dialog(handle: AppHandle) -> Response {
    let mut visible = false;
    // Create a channel to receive the result from the pick_folder closure
    let (sender, receiver) = oneshot::channel();

    let selector = handle.windows().get("selector").cloned().unwrap();
    if selector.is_visible().unwrap() {
        visible = true;
        selector.hide().unwrap();
    }

    // Spawn a tokio task to run the pick_folder closure
    task::spawn(async move {
        FileDialogBuilder::new().pick_folder(move |folder_path| {
            let result =
                match folder_path {
                    Some(path) => {
                        if cfg!(target_os="windows") {
                            Response::new(Some(format!("{}\\", path.to_string_lossy().to_string())), None)
                        }
                        else {
                            Response::new(Some(format!("{}/", path.to_string_lossy().to_string())), None)
                        }
                    }
                    None => Response::new(None, Some("The path is empty.".to_string()))
                };

// Send the result through the channel
            sender.send(result).unwrap();
        });
    });

    if visible {
        selector.show().unwrap();
    }

    // Await the result from the channel and return it
    receiver.await.unwrap_or_else( |_| Response::new( None, Some(format!("Failed to retrieve the folder path."))))
}

#[tauri::command]
pub async fn current_default_path() -> Response {
    let mut result;
    #[cfg(target_os = "windows")] {
        result = format!("{}\\", env::var("USERPROFILE").unwrap().to_string());
    }
    #[cfg(target_os = "linux")] {
        result = format!("{}/", env::var("HOME").unwrap().to_string());
    }
    #[cfg(target_os = "macos")] {
        let output = Command::new("defaults")
            .args(&["read", "com.apple.screencapture", "location"])
            .output()
            .await
            .expect("Failed to execute command");

        result = String::from_utf8(output.stdout).unwrap().trim().to_string();

        if result.is_empty() { result = env::var("HOME").unwrap().to_string(); }

        if result.starts_with('~') { result = result.replace("~", env::var("HOME").unwrap().as_str()) }

        if !result.ends_with("/") { result.push('/'); }
    }

    return Response::new(Some(result), None );
}

pub fn utils_dir() -> String {
    if !cfg!(target_os="windows") { env::var("HOME").unwrap() + "/.screencrab" }
    else { env::var("APPDATA").unwrap() + "/.screencrab" }
}

pub fn get_current_monitor_index(window: &Window) -> usize {
    window.available_monitors()
        .unwrap()
        .into_iter()
        .position(|item| item.name().unwrap().eq(window.current_monitor().unwrap().unwrap().name().unwrap()))
        .unwrap_or(0) + 1
}

#[cfg(target_os = "windows")]
pub fn get_monitor_position(window: &Window, index: usize) -> PhysicalPosition<i32> {
    let mut position = PhysicalPosition::new(0, 0);
    for (i, monitor) in window.available_monitors().unwrap().iter().enumerate() {
        if i >= index { break; }
        let monitor_size = monitor.size();
        position.x += monitor_size.width as i32;
    }
    position
}

pub fn monitor_dialog(app: AppHandle) {
    MessageDialogBuilder::new("New Monitor Detected", "ScreenCrab detected a new monitor. Please Restart the application in order to re-index all monitors.")
        .kind(MessageDialogKind::Error)
        .buttons(MessageDialogButtons::OkCancelWithLabels("Restart".to_string(), "Quit".to_string()))
        .show(move |value| {
            if value { app.restart(); }
            else { app.exit(0);  }
        });
}

#[tauri::command]
pub fn custom_area_selection(app: AppHandle, id: String, left: f64, top: f64, width: f64, height: f64) {
    let offset = LogicalPosition::new(app.windows().get(id.as_str()).unwrap().outer_position().unwrap().x as f64,app.windows().get(id.as_str()).unwrap().outer_position().unwrap().y as f64);
    let scale_factor = app.windows().get(id.as_str()).unwrap().current_monitor().unwrap().unwrap().scale_factor();
    let position = LogicalPosition::new((left + offset.x/scale_factor) as i32, (top + offset.y/scale_factor) as i32);
    let size = LogicalSize::new(width as i32, height as i32);

    let n = app.windows().get("main_window").unwrap().available_monitors().unwrap().len();
    for i in 0..n {
        if let Some(helper) = app.windows().get(format!("helper_{}", i).as_str()) {
            helper.hide().unwrap();
        }
        else {
            monitor_dialog(app.app_handle());
        }
    }

    app.windows().get("selector").unwrap().set_size(size).unwrap();
    app.windows().get("selector").unwrap().set_position(position).unwrap();
    app.windows().get("selector").unwrap().show().unwrap();
    app.windows().get("main_window").unwrap().set_focus().unwrap();

}

#[tauri::command]
pub fn show_all_helpers(app: AppHandle) {
    app.windows().get("selector").unwrap().hide().unwrap();
    let monitors = app.windows().get("main_window").unwrap().available_monitors().unwrap();
    for (i, monitor) in monitors.iter().enumerate() {
        if let Some(helper) = app.windows().get(format!("helper_{}", i).as_str()) {
            helper.set_position(monitor.position().to_logical::<f64>(monitor.scale_factor())).unwrap();
            helper.set_size(monitor.size().to_logical::<f64>(monitor.scale_factor())).unwrap();
            helper.show().unwrap();
        }
        else {
            monitor_dialog(app.app_handle());
        }
    }
}

#[tauri::command]
pub fn hide_all_helpers(app: AppHandle) {
    app.windows().get("selector").unwrap().hide().unwrap();
    let monitors = app.windows().get("main_window").unwrap().available_monitors().unwrap();
    for i in 0..monitors.len() {
        if let Some(helper) = app.windows().get(format!("helper_{}", i).as_str()) {
            helper.hide().unwrap();
        }
        else {
            monitor_dialog(app.app_handle());
        }
    }
}

#[cfg(not(target_os = "macos"))]
pub fn copy_to_clipboard(path: String) -> Result<(), Error> {
    let mut clip = arboard::Clipboard::new().unwrap();
    let img = image::open(path).unwrap();
    let pixels = img
        .pixels()
        .into_iter()
        .map(|(_, _, pixel)| pixel.0)
        .flatten()
        .collect::<Vec<_>>();
    let img_data = arboard::ImageData {
        height: img.height() as usize,
        width: img.width() as usize,
        bytes: Cow::Owned(pixels),
    };
    clip.set_image(img_data)
}
