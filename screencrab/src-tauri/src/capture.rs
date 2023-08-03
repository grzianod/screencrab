use tauri::api::dialog::FileDialogBuilder;
use serde::{Serialize, Deserialize};
use tokio::task;
use tokio::sync::oneshot;
use tokio::process::Command;
use tauri::{Window, AppHandle, Manager, PhysicalPosition};
use tauri::PhysicalSize;
use std::process::Stdio;
use tauri_api::path::home_dir;


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

pub fn cwd() -> Response {
    if let Some(home_dir) = home_dir() {
        Response { response: Some(home_dir.display().to_string()), error: None }
    } else {
        Response { response: None, error: Some(format!("Error while retrieving current user home directory.")) }
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
            return Response { response: Some(format!("Screen Crab saved to Clipboard")), error: None };
        }
    return Response { response: None, error: Some(format!("Screen Crab cancelled.")) };
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
        return Response { response: Some(format!("Screen Crab saved to Clipboard")), error: None };
    }
    return Response { response: None, error: Some(format!("Screen Crab cancelled.")) };
}

pub async fn record_fullscreen(window: Window, filename: &str, timer: u64, pointer: bool, clipboard: bool) -> Response {
    let filename1 = filename.to_string();
    let filename2 = filename.to_string();

        let mut command = Command::new("screencapture");
        command.stdin(Stdio::piped());
        command.arg("-v");

        if pointer { command.arg("-C"); }
        if clipboard { command.arg("-c"); }

        command.args(&["-T", timer.to_string().as_str()]);

        let mut process = command.arg(filename1.as_str()).spawn().map_err(|e| Response {response: None, error: Some(format!("Failed to launch screen record: {}",e))}).unwrap();
        let _stdin = process.stdin.take().unwrap();  //do not release process stdin at wait(), capture it to send SIGTERM to recording process
        let pid = process.id().unwrap();

        window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            let _output = Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()
                .await;
            });
        });

        window.listen_global("stop", move |_event| {
            tokio::task::spawn(async move {
                let _output = Command::new("kill")
                    .arg("-2")  //SIGTERM
                    .arg(pid.to_string())
                    .output()
                    .await;
            });
        });

    let output = process.wait().await.unwrap();
    if output.success() {
        // Use tokio::task::spawn to execute the opening
        let _open_task = task::spawn(async move {
            let _open = Command::new("open").arg(filename2.as_str()).output().await.map_err(|e| Response { response: None, error: Some(format!("Failed to open screenshot: {}", e)) });
        });
        return Response { response: Some(format!("Screen Crab saved to {}", filename1.to_string())), error: None };
    }
    return Response { response: None, error: Some(format!("Screen Crab cancelled.")) };

}

pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, pointer: bool, clipboard: bool) -> Response {
    let filename1 = filename.to_string();
    let filename2 = filename.to_string();

    let mut command = Command::new("screencapture");
    command.stdin(Stdio::piped());
    command.arg("-v");

    if pointer { command.arg("-C"); }
    if clipboard { command.arg("-c"); }

    command.args(&["-T", timer.to_string().as_str()]);
    command.args(&["-R", area]);

    let mut process = command.arg(filename1.as_str()).spawn().map_err(|e| Response {response: None, error: Some(format!("Failed to launch screen record: {}",e))}).unwrap();
    let mut _stdin = process.stdin.take().unwrap();  //do not release process stdin at wait(), capture it to send SIGTERM to recording process
    let pid = process.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            let _output = Command::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()
                .await;
        });
    });

    window.listen_global("stop", move |_event| {
        tokio::task::spawn(async move {
            let _output = Command::new("kill")
                .arg("-2")  //SIGTERM
                .arg(pid.to_string())
                .output()
                .await;
        });
    });

    let output = process.wait().await.unwrap();
    if output.success() {
        // Use tokio::task::spawn to execute the opening
        let _open_task = task::spawn(async move {
            let _open = Command::new("open").arg(filename2.as_str()).output().await.map_err(|e| Response { response: None, error: Some(format!("Failed to open screenshot: {}", e)) });
        });
        return Response { response: Some(format!("Screen Crab saved to {}", filename1.to_string())), error: None };
    }
    return Response { response: None, error: Some(format!("Screen Crab cancelled.")) };
}
