use std::env;
use tauri::api::dialog::FileDialogBuilder;
use serde::{Serialize, Deserialize};
use tokio::task;
use tokio::sync::oneshot;
use tokio::process::Command;
use tauri::{Window, AppHandle, Manager, PhysicalPosition};
use tauri::PhysicalSize;


#[derive(Clone, serde::Serialize)]
struct Payload {
    position: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    response: Option<String>,
    error: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Position {
    position: PhysicalPosition<i32>,
    size: PhysicalSize<u32>
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

    if visible {
        selector.show().unwrap();
    }

// Await the result from the channel and return it
    receiver.await.unwrap_or_else(|_| Response {
        response: None,
        error: Some("Failed to retrieve the folder path.".to_string()),
    })
}

pub async fn capture_fullscreen(window: Window, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool) -> Response {
    let filename1 = filename.to_string();

        let mut command = Command::new("screencapture");

        if pointer { command.arg("-C"); }
        if clipboard { command.arg("-c"); }

        command.args(&["-t", file_type]);
        command.args(&["-T", timer.to_string().as_str()]);

        let process = command.arg(filename1.as_str()).spawn().map_err(|e| Response { response: None, error: Some(format!("Failed to take screenshot: {}", e)) });
        let pid = process.as_ref().unwrap().id().unwrap();

        window.listen_global("kill", move |_event| {
            tokio::task::spawn(async move {
                let _output = Command::new("kill")
                    .arg("-15")
                    .arg(pid.to_string())
                    .output()
                    .await;
            });
        });

        let output = process.unwrap().wait().await.unwrap();
        if output.success() {
            if !clipboard {
                // Use tokio::task::spawn to execute the opening
                let _open_task = task::spawn(async move {
                    let _open = Command::new("open").arg(filename1.as_str()).output().await.map_err(|e| Response { response: None, error: Some(format!("Failed to open screenshot: {}", e)) });
                });
                return Response { response: Some(format!("Screen Crab saved to {}", filename.to_string())), error: None };
            }
            Response { response: Some(format!("Screen Crab saved to Clipboard")), error: None }
        } else {
            Response { response: None, error: Some(format!("Screen Crab cancelled.")) }
        }
}

pub async fn capture_custom(window: Window, area: &str, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool) -> Response {
    let filename1 = filename.to_string();

    let mut command = Command::new("screencapture");

    if pointer { command.arg("-C"); }
    if clipboard { command.arg("-c"); }

    command.args(&["-R", area]);
    command.args(&["-t", file_type]);
    command.args(&["-T", timer.to_string().as_str()]);


    let process = command.arg(filename1.as_str()).spawn().map_err(|e| Response { response: None, error: Some(format!("Failed to take screenshot: {}", e)) });
    let pid = process.as_ref().unwrap().id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            let _output = Command::new("kill")
                .arg("-15")
                .arg(pid.to_string())
                .output()
                .await;
        });
    });

    let output = process.unwrap().wait().await.unwrap();
    if output.success() {
        if !clipboard {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = Command::new("open").arg(filename1.as_str()).output().await.map_err(|e| Response { response: None, error: Some(format!("Failed to open screenshot: {}", e)) });
            });
            return Response { response: Some(format!("Screen Crab saved to {}", filename.to_string())), error: None };
        }
        Response { response: Some(format!("Screen Crab saved to Clipboard")), error: None }
    } else {
        Response { response: None, error: Some(format!("Screen Crab cancelled.")) }
    }
}

pub async fn record_fullscreen(window: Window, filename: &str, timer: u64, pointer: bool, clipboard: bool) -> Response {
    let filename1 = filename.to_string();
    let filename2 = filename.to_string();

    let record_task = tokio::task::spawn(async move {
        let mut command = Command::new("screencapture");

        command.arg("-v");

        if pointer { command.arg("-C"); }
        if clipboard { command.arg("-c"); }

        command.args(&["-T", timer.to_string().as_str()]);

        let process = command.arg(filename1.as_str()).spawn().map_err(|e| Response { response: None, error: Some(format!("Failed to take screenshot: {}", e)) });
        let pid = process.as_ref().unwrap().id().unwrap();

        window.listen_global("stop", move |_event| {
            tokio::task::spawn(async move {
                let _output = Command::new("kill")
                    .arg("-2")
                    .arg(pid.to_string())
                    .output()
                    .await;
            });
        });

        let output = process.unwrap().wait().await.unwrap();
        if output.success() {
            if !clipboard { return Response { response: Some(format!("Screen Crab saved to {}", filename1.to_string())), error: None }; }
            else { return Response { response: None, error: Some(format!("Screen Crab cancelled.")) }; }
        }
        return Response { response: None, error: Some(format!("Failed to take Screen Crab.")) };
    });

    let output = record_task.await.unwrap();
    if !clipboard {
        // Use tokio::task::spawn to execute the opening
        let _open_task = task::spawn(async move {
            let _open = Command::new("open").arg(filename2.as_str()).output().await.map_err(|e| Response { response: None, error: Some(format!("Failed to open screenshot: {}", e)) });
        });
    }
    output
}

pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, pointer: bool, clipboard: bool) -> Response {
    let filename1 = filename.to_string();
    let filename2 = filename.to_string();
    let area1 = area.to_string();

    window.emit("selected", {}).unwrap();
    window.listen_global("position", move |event| {
        let position = event.payload().unwrap();
        println!("{}", position);
    });

    let record_task = tokio::task::spawn(async move {
        let mut command = Command::new("screencapture");

        command.arg("-v");
        command.args(&["-R", area1.as_str()]);

        if pointer { command.arg("-C"); }
        if clipboard { command.arg("-c"); }

        command.args(&["-T", timer.to_string().as_str()]);

        let process = command.arg(filename1.as_str()).spawn().map_err(|e| Response { response: None, error: Some(format!("Failed to take screenshot: {}", e)) });
        let pid = process.as_ref().unwrap().id().unwrap();

        window.listen_global("stop", move |_event| {
            tokio::task::spawn(async move {
                let _output = Command::new("kill")
                    .arg("-2")
                    .arg(pid.to_string())
                    .output()
                    .await;
            });
        });

        let output = process.unwrap().wait().await.unwrap();
        if output.success() {
            if !clipboard { return Response { response: Some(format!("Screen Crab saved to {}", filename1.to_string())), error: None }; }
            else { return Response { response: None, error: Some(format!("Screen Crab cancelled.")) }; }
        }
        return Response { response: None, error: Some(format!("Failed to take Screen Crab.")) };
    });

    let output = record_task.await.unwrap();
    if !clipboard {
        // Use tokio::task::spawn to execute the opening
        let _open_task = task::spawn(async move {
            let _open = Command::new("open").arg(filename2.as_str()).output().await.map_err(|e| Response { response: None, error: Some(format!("Failed to open screenshot: {}", e)) });
        });
    }
    output
}
