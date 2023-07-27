use std::env;
use tauri::api::dialog::FileDialogBuilder;
use serde::{Serialize, Deserialize};
use tokio::task;
use tokio::sync::oneshot;
use tokio::process::Command;
use tokio::task::JoinHandle;
use tokio::sync::Mutex;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    static ref CAPTURE_PROCESS: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    response: Option<String>,
    error: Option<String>,
}

impl Response {
    // Constructor function to create a new instance of Response
    pub fn new(response: Option<String>, error: Option<String>) -> Self {
        Response { response, error }
    }
}

pub async fn cwd() -> Response {
    match env::current_dir() {
        Ok(current_dir) => {
            Response { response: Some(current_dir.display().to_string()), error: None }
        }
        Err(err) => {
            Response { response: None, error: Some(format!("Error getting current working directory: {}", err)) }
        }
    }
}


pub async fn folder_dialog() -> Response {
// Create a channel to receive the result from the pick_folder closure
    let (sender, receiver) = oneshot::channel();

// Spawn a blocking task to run the pick_folder closure
    task::spawn_blocking(move || {
        FileDialogBuilder::new().pick_folder(move |folder_path| {
            let result = match folder_path {
                Some(path) => Response {
                    response: Some(path.to_string_lossy().to_string()),
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

// Await the result from the channel and return it
    receiver.await.unwrap_or_else(|_| Response {
        response: None,
        error: Some("Failed to retrieve the folder path.".to_string()),
    })
}

pub async fn capture_screen(filename: &str, file_type: &str, view: &str, timer: u64, pointer: bool, clipboard: bool) -> Response {
    let mut command = Command::new("screencapture");

    match view {
        "fullscreen" => {}
        "window" => { command.arg("-w"); }
        "custom" => { command.arg("-i"); }
        _ => { return Response { response: None, error: Some(format!("Invalid view {}", view))} }
    }

    if pointer { command.arg("-C"); }
    if clipboard { command.arg("-c"); }

    command.args(&["-t", file_type]);
    command.args(&["-T", timer.to_string().as_str()]);


    let output = command.arg(filename).output().await.map_err(|e| Response { response: None, error: Some(format!("Failed to take screenshot: {}", e))});

    if !clipboard {
        let open = Command::new("open").args(&["-a", "Preview", filename]).output().await.map_err(|e| Response { response: None, error: Some(format!("Failed to open screenshot: {}", e))});
    }

    Response { response: Some(filename.to_string()), error: None }
}


pub async fn cancel() -> Response {
    Response { response: Some(format!("Capture cancelled")), error: None }
}


pub async fn record_screen(filename: &str) -> Response {
    Response { response: Some(filename.to_string()), error: None }
}
