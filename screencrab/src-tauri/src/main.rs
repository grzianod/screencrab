mod lib;

use tauri::api::dialog::FileDialogBuilder;
use tauri::{AppHandle, State};
use tauri::Manager;
use serde::{Serialize, Deserialize};
use tokio::task;
use tokio::sync::oneshot;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Response {
    response: Option<String>,
    error: Option<String>,
}

#[tauri::command]
async fn folder_dialog() -> Response {
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

#[tauri::command]
fn capture() {
    let filename = "screenshot.png";
    lib::capture_screen(filename).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![capture, folder_dialog])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
