// File: src/capture

use std::process::Command;
use std::io::{Result, Error, ErrorKind};
use std::env;
use tauri::api::dialog::FileDialogBuilder;
use serde::{Serialize, Deserialize};
use tokio::task;
use tokio::sync::oneshot;


#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    response: Option<String>,
    error: Option<String>,
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

pub fn capture_screen(filename: &str, file_type: &str, view: &str, pointer: bool, clipboard: bool) -> Result<()> {
    let mut command = Command::new("screencapture");

    match view {
        "fullscreen" => {}
        "window" => { command.arg("-w"); }
        "custom" => { command.arg("-i"); }
        _ => { return Err(Error::new(ErrorKind::Other, format!("Invalid view: {:?}", view))); } /* TODO: handle error */
    }

    if pointer { command.arg("-C"); }
    if clipboard { command.arg("-c"); }

    command.args(&["-t", file_type]);

    let output = command.arg(filename).output()?;
    if !output.status.success() {
        return Err(Error::new(ErrorKind::Other, format!("Failed to take a screenshot")));
    }

    if !clipboard {
        let open = Command::new("open").args(&["-a", "Preview", filename]).output()?;
        if !open.status.success() {
            return Err(Error::new(ErrorKind::Other, format!("Failed to open a screenshot")));
        }
    }
    Ok(())
}


pub fn record_screen(filename: &str) -> Result<()> {
    let output = Command::new("screencapture")
        .args(&["-v", filename])
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, format!("Failed to take a screenshot: {:?}", output)))
    }
}
