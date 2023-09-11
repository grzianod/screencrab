use serde::{Serialize, Deserialize};
use tokio::sync::oneshot;
use std::{env};
use tauri::api::dialog::FileDialogBuilder;
use tokio::task;
use tokio::process::Command;
use tauri::{AppHandle, Window, Manager};

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

pub async fn folder_picker(handle: AppHandle) -> Response {
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

pub async fn current_default_path() -> Response {
    let mut result;
    #[cfg(target_os = "windows")] {
        result = format!("{}\\", env::home_dir().unwrap().display());
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

pub fn get_current_monitor_index(window: &Window) -> usize {
    window.available_monitors()
        .unwrap()
        .into_iter()
        .position(|item| item.name().unwrap().eq(window.current_monitor().unwrap().unwrap().name().unwrap()))
        .unwrap_or(0) + 1
}

