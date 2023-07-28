use std::env;
use tauri::api::dialog::FileDialogBuilder;
use serde::{Serialize, Deserialize};
use tokio::task;
use tokio::sync::oneshot;
use tokio::process::Command;

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
    let filename1 = filename.to_string();
    let file_type = file_type.to_string();
    let view = view.to_string();

    // Use tokio::task::spawn to execute the capture_screen asynchronously
    let capture_task = task::spawn(async move {
        let mut command = Command::new("screencapture");

        match view.as_str() {
            "fullscreen" => {}
            "window" => { command.arg("-w"); }
            "custom" => { command.arg("-i"); }
            _ => { return Response { response: None, error: Some(format!("Invalid view {}", view)) } }
        }

        if pointer { command.arg("-C"); }
        if clipboard { command.arg("-c"); }

        command.args(&["-t", file_type.as_str()]);
        command.args(&["-T", timer.to_string().as_str()]);

        let _output = command.arg(filename1.as_str()).output().await.map_err(|e| Response { response: None, error: Some(format!("Failed to take screenshot: {}", e)) });

        Response { response: Some(filename1), error: None }
    });

    // Wait for the capture task to complete and return its value
    capture_task.await.unwrap_or_else(|e| Response::new(None, Some(format!("Failed to take screenshot: {}",e))));

    let filename2 = filename.to_string();
    if !clipboard {

            // Use tokio::task::spawn to execute the opening
            let open_task = task::spawn(async move {
                let _open = Command::new("open").args(&["-a", "Preview", filename2.as_str()]).output().await.map_err(|e| Response { response: None, error: Some(format!("Failed to open screenshot: {}", e)) });
                Response { response: Some(filename2), error: None }
            });

        // Wait for the open task to complete and return its value
        open_task.await.unwrap_or_else(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e)) ));
    }

    let filename3 = filename.to_string();
    Response { response: Some(filename3), error: None }
}


pub async fn record_screen(filename: &str) -> Response {
    Response { response: Some(filename.to_string()), error: None }
}