use std::{env};
use tauri::api::dialog::FileDialogBuilder;
use serde::{Serialize, Deserialize};
use tokio::task;
use tokio::sync::oneshot;
use tokio::process::Command;
use tauri::{Window, AppHandle, Manager, PhysicalPosition};
use tauri::PhysicalSize;
use tauri::api::notification::Notification;
use std::fs;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    response: Option<String>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    position: PhysicalPosition<i32>,
    size: PhysicalSize<u32>,
}

impl Response {
    // Constructor function to create a new instance of Response
    pub fn new(response: Option<String>, error: Option<String>) -> Self {
        Response { response, error }
    }
}

pub async fn current_default_path() -> Response {
    let result = env::var("HOME").unwrap().to_string();

    return Response { response: Some(result), error: None };
}

pub async fn folder_dialog(handle: AppHandle) -> Response {
    let mut visible = false;
// Create a channel to receive the result from the pick_folder closure
    let (sender, receiver) = oneshot::channel();

    let selector = handle.windows().get("selector").cloned().unwrap();
    if selector.is_visible().unwrap() {
        visible = true;
        selector.hide().unwrap();
    }

// Spawn a blocking task to run the pick_folder closure
    task::spawn_blocking(move || {
        FileDialogBuilder::new().pick_folder(move |folder_path| {
            let result = match folder_path {
                Some(path) => Response {
                    response: Some(format!("{}\\", path.to_string_lossy().to_string())),
                    error: None,
                },
                None => Response {
                    response: None,
                    error: Some("The path is empty.".to_string()),
                },
            };

// Send the result through the channel
            sender.send(result).unwrap();
        });
    });

    if visible {
        selector.show().unwrap();
    }

// Await the result from the channel and return it
    receiver.await.unwrap_or_else(|_| Response {
        response: None,
        error: Some("Failed to retrieve the folder path.".to_string()),
    })
}

fn get_current_monitor_index(window: &Window) -> usize {
    window.available_monitors()
        .unwrap()
        .into_iter()
        .position(|item| item.name().unwrap().eq(window.current_monitor().unwrap().unwrap().name().unwrap()))
        .unwrap_or(0) + 1
}

pub async fn capture_fullscreen(app: AppHandle, window: Window, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {

    const SCRIPT: &[u8] = include_bytes!("screenshot_full_script.ps1");
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("screenshot_full_script.ps1");

    {
        let mut temp_file = fs::File::create(&temp_file_path).unwrap();
        temp_file.write_all(SCRIPT).unwrap();
    }


    let mut process = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(temp_file_path.clone())
        .arg("-filename")
        .arg(filename)
        .arg("-filetype")
        .arg(file_type)
        .arg("-timer")
        .arg(timer.to_string())
        .arg("-pointer")
        .arg(if pointer { "1" } else { "0" })  // Convert to "1" or "0"
        .arg("-clipboard")
        .arg(if clipboard { "1" } else { "0" })  // Convert to "1" or "0"
        .arg("-openfile")
        .arg(if open_file { "1" } else { "0" })  // Convert to "1" or "0"
        .spawn()
        .unwrap();

    let pid = process.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
           Command::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().await;
        });
    });

    let mut output = process.wait().await.unwrap();
    fs::remove_file(&temp_file_path).unwrap();
    if output.success() {
        if clipboard {
            Notification::new(&app.config().tauri.bundle.identifier)
                .title("All done!")
                .body("Screen Crab saved to Clipboard")
                .icon("icons/icon.icns").show().unwrap();
            return Response { response: Some(format!("Screen Crab saved to Clipboard")), error: None }; }
        else {
            Notification::new(&app.config().tauri.bundle.identifier)
                .title("All done!")
                .body(format!("Screen Crab saved to {}", filename.to_string()))
                .icon("icons/icon.icns").show().unwrap();
            return Response { response: Some(format!("Screen Crab saved to {}", filename.to_string())), error: None }; }
    }
    Notification::new(&app.config().tauri.bundle.identifier)
        .body("Screen Crab cancelled")
        .icon("icons/icon.icns").show().unwrap();
    return Response { response: None, error: Some(format!("Screen Crab cancelled")) };
}

pub async fn capture_custom(app: AppHandle, window: Window, area: &str, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    
    const SCRIPT: &[u8] = include_bytes!("screenshot_custom_script.ps1");
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("screenshot_custom_script.ps1");

    {
        let mut temp_file = fs::File::create(&temp_file_path).unwrap();
        temp_file.write_all(SCRIPT).unwrap();
    }

    let mut process = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(temp_file_path.clone())  // Assuming there's a custom script for this.
        .arg("-area")
        .arg(area)  // Pass the custom area string
        .arg("-filename")
        .arg(filename)
        .arg("-filetype")
        .arg(file_type)
        .arg("-timer")
        .arg(timer.to_string())
        .arg("-pointer")
        .arg(if pointer { "1" } else { "0" })  // Convert to "1" or "0"
        .arg("-clipboard")
        .arg(if clipboard { "1" } else { "0" })  // Convert to "1" or "0"
        .arg("-openfile")
        .arg(if open_file { "1" } else { "0" })  // Convert to "1" or "0"
        .spawn()
        .unwrap();


    let pid = process.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            Command::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().await;
        });
    });

    let output = process.wait().await.unwrap();
    fs::remove_file(&temp_file_path).unwrap();
    if output.success() {
        if clipboard {
            Notification::new(&app.config().tauri.bundle.identifier)
                .title("All done!")
                .body("Screen Crab saved to Clipboard")
                .icon("icons/icon.icns").show().unwrap();
            return Response { response: Some(format!("Screen Crab saved to Clipboard")), error: None }; }
        else {
            Notification::new(&app.config().tauri.bundle.identifier)
                .title("All done!")
                .body(format!("Screen Crab saved to {}", filename.to_string()))
                .icon("icons/icon.icns").show().unwrap();
            return Response { response: Some(format!("Screen Crab saved to {}", filename.to_string())), error: None }; }
    }
    Notification::new(&app.config().tauri.bundle.identifier)
        .body("Screen Crab cancelled")
        .icon("icons/icon.icns").show().unwrap();
    return Response { response: None, error: Some(format!("Screen Crab cancelled")) };
}


pub async fn record_fullscreen(app: AppHandle, window: Window, filename: &str, timer: u64, _pointer: bool, clipboard: bool, audio: bool, open_file: bool) -> Response {
    
    const SCRIPT: &[u8] = include_bytes!("record_full_script.ps1");
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("record_full_script.ps1");

    {
        let mut temp_file = fs::File::create(&temp_file_path).unwrap();
        temp_file.write_all(SCRIPT).unwrap();
    }

    let mut process = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(temp_file_path.clone())  // Name of the screen recording script
        .arg("-filename")
        .arg(filename)
        .arg("-timer")
        .arg(timer.to_string())  // Convert u64 to String
        .arg("-audio")
        .arg(if audio { "1" } else { "0" })  // Convert to "1" or "0"
        .arg("-openfile")
        .arg(if open_file { "1" } else { "0" })  // Convert to "1" or "0"
        .spawn()
        .unwrap();

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let pid = process.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            Command::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().await;
        });
    });

    let window_ = window.clone();
    let pid2 = process.id().unwrap();

    window.listen_global("stop", move |_event| {
        tokio::task::spawn(async move {
            Command::new("taskkill").args(&["/PID", &pid2.to_string()]).output().await;
        });
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let output = process.wait().await.unwrap();
    fs::remove_file(&temp_file_path).unwrap();
    if output.success() {
        if clipboard {
            Notification::new(&app.config().tauri.bundle.identifier)
                .title("All done!")
                .body("Screen Crab saved to Clipboard")
                .icon("icons/icon.icns").show().unwrap();
            return Response { response: Some(format!("Screen Crab saved to Clipboard")), error: None }; }
        else {
            Notification::new(&app.config().tauri.bundle.identifier)
                .title("All done!")
                .body(format!("Screen Crab saved to {}", filename.to_string()))
                .icon("icons/icon.icns").show().unwrap();
            return Response { response: Some(format!("Screen Crab saved to {}", filename.to_string())), error: None }; }
    }
    Notification::new(&app.config().tauri.bundle.identifier)
        .body("Screen Crab cancelled")
        .icon("icons/icon.icns").show().unwrap();
    return Response { response: None, error: Some(format!("Screen Crab cancelled")) };
}


pub async fn record_custom(app: AppHandle, window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    return Response { response: Some(format!("Screen Crab taken!")), error: None };
}
