use std::{env};
use tauri::api::dialog::FileDialogBuilder;
use serde::{Serialize, Deserialize};
use tokio::task;
use tokio::sync::oneshot;
use tokio::process::Command;
use tauri::{Window, AppHandle, Manager, PhysicalPosition};
use tauri::PhysicalSize;
use std::process::Stdio;
use tauri::api::notification::Notification;

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
                    response: Some(format!("{}/", path.to_string_lossy().to_string())),
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
    let output = Command::new("powershell")
        .arg("-File")
        .arg("src/screenshot_script.ps1")
        .output()
        .await;

    match output {
        Ok(o) => {
            if o.status.success() {
                Response {
                    response: Some("Screenshot captured successfully!".to_string()),
                    error: None,
                }
            } else {
                let error_message = String::from_utf8_lossy(&o.stderr);
                Response {
                    response: None,
                    error: Some(format!("Failed to capture screenshot: {}", error_message)),
                }
            }
        }
        Err(e) => Response {
            response: None,
            error: Some(format!("Failed to run PowerShell script: {}", e)),
        },
    }
}

pub async fn capture_custom(app: AppHandle, window: Window, area: &str, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    return Response { response: Some(format!("Screen Crab taken!")), error: None };
}

pub async fn record_fullscreen(app: AppHandle, window: Window, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    return Response { response: Some(format!("Screen Crab taken!")), error: None };
}

pub async fn record_custom(app: AppHandle, window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    return Response { response: Some(format!("Screen Crab taken!")), error: None };
}
