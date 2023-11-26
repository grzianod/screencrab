use serde::{Serialize, Deserialize};
use tokio::sync::oneshot;
use tauri::api::dialog::FileDialogBuilder;

use tokio::task;
use tokio::process::Command;
use tauri::{AppHandle, Window, Manager, App};
use std::{env, fs};
use std::fs::File;
use std::io::Write;
use tauri::api::dialog::{MessageDialogBuilder, MessageDialogButtons, MessageDialogKind};

#[cfg(not(target_os = "macos"))]
use arboard::{Clipboard, ImageData};
#[cfg(not(target_os = "macos"))]
use image::{DynamicImage, GenericImageView};
#[cfg(not(target_os = "macos"))]
use crate::get_image_bytes;
#[cfg(not(target_os = "macos"))]
use std::io::Read;
#[cfg(not(target_os = "macos"))]
use std::borrow::Cow;
#[cfg(not(target_os = "macos"))]
use std::io::Cursor;


// the payload type must implement `Serialize` and `Clone`.
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

pub fn utils_dir() -> String {
    if !cfg!(target_os="windows") { env::var("HOME").unwrap() + "/.screencrab" }
    else { env::var("APPDATA").unwrap() + "/.screencrab" }
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

pub fn get_current_monitor_index(window: &Window) -> usize {
    window.available_monitors()
        .unwrap()
        .into_iter()
        .position(|item| item.name().unwrap().eq(window.current_monitor().unwrap().unwrap().name().unwrap()))
        .unwrap_or(0) + 1
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

#[cfg(not(target_os = "macos"))]
pub fn copy_to_clipboard(path: String) -> Response {
    let mut ctx = Clipboard::new().unwrap();
    let mut file = File::open(path).unwrap();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let image: DynamicImage = image::load_from_memory(&buffer).unwrap();

    let format = image::guess_format(&buffer)
        .map(|f| f.into())
        .unwrap_or(image::ImageFormat::Png);

    let (width, height) = image.dimensions();

    let mut image_bytes = vec![];
    image.write_to(&mut Cursor::new(&mut image_bytes), format).unwrap();

    let image_data = ImageData { width: width as usize, height: height as usize, bytes: Cow::from(image_bytes) };
    if let Err(err) = ctx.set_image(image_data) {
        return Response::new(None, Some(err.to_string()));
    }
    else {
        return Response::new(Some(format!("Screen Crab saved to Clipboard")), None);
    }
}