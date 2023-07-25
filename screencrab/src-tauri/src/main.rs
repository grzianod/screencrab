use crate::lib::Response;

mod lib;

#[tauri::command]
async fn folder_dialog() -> Response {
    lib::folder_dialog().await
}

#[tauri::command]
async fn cwd() -> Response {
    lib::cwd().await
}


#[tauri::command]
fn capture() {
    let filename = "screenshot.png";
    lib::capture_screen(filename).unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![capture, folder_dialog, cwd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
