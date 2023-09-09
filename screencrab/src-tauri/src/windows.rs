use tokio::task;
use tokio::process::Command;
use tauri::{Window, Manager};
use std::process::Stdio;
use crate::utils::*;

pub async fn capture_fullscreen(window: Window, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {

    const SCRIPT: &[u8] = include_bytes!("screenshot_full_script.ps1");
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("screenshot_full_script.ps1");

    {
        let mut temp_file = fs::File::create(&temp_file_path).unwrap();
        temp_file.write_all(SCRIPT).unwrap();
    }


    let mut process = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(temp_file_path.clone())
        .arg("-filename")
        .arg(filename)
        .arg("-filetype")
        .arg(file_type)
        .arg("-timer")
        .arg(timer.to_string())
        .arg("-pointer")
        .arg(if pointer { "1" } else { "0" })  // Convert to "1" or "0"
        .arg("-clipboard")
        .arg(if clipboard { "1" } else { "0" })  // Convert to "1" or "0"
        .arg("-openfile")
        .arg(if open_file { "1" } else { "0" })  // Convert to "1" or "0"
        .spawn()
        .unwrap();

    let pid = process.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
           Command::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().await;
        });
    });

    let mut output = process.wait().await.unwrap();
    fs::remove_file(&temp_file_path).unwrap();
    if output.success() {
        if clipboard {
            return Response::new(Some(format!("Screen Crab saved to Clipboard")), None ); }
        else {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), error: None ); }
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}

pub async fn capture_custom(window: Window, area: &str, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    
    const SCRIPT: &[u8] = include_bytes!("screenshot_custom_script.ps1");
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("screenshot_custom_script.ps1");

    {
        let mut temp_file = fs::File::create(&temp_file_path).unwrap();
        temp_file.write_all(SCRIPT).unwrap();
    }

    let mut process = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(temp_file_path.clone())  // Assuming there's a custom script for this.
        .arg("-area")
        .arg(area)  // Pass the custom area string
        .arg("-filename")
        .arg(filename)
        .arg("-filetype")
        .arg(file_type)
        .arg("-timer")
        .arg(timer.to_string())
        .arg("-pointer")
        .arg(if pointer { "1" } else { "0" })  // Convert to "1" or "0"
        .arg("-clipboard")
        .arg(if clipboard { "1" } else { "0" })  // Convert to "1" or "0"
        .arg("-openfile")
        .arg(if open_file { "1" } else { "0" })  // Convert to "1" or "0"
        .spawn()
        .unwrap();


    let pid = process.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            Command::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().await;
        });
    });

    let output = process.wait().await.unwrap();
    fs::remove_file(&temp_file_path).unwrap();
    if output.success() {
        if clipboard {
            return Response::new(Some(format!("Screen Crab saved to Clipboard")), None ); }
        else {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None ); }
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}


pub async fn record_fullscreen(window: Window, filename: &str, timer: u64, _pointer: bool, clipboard: bool, audio: bool, open_file: bool) -> Response {
    
    const SCRIPT: &[u8] = include_bytes!("record_full_script.ps1");
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("record_full_script.ps1");

    {
        let mut temp_file = fs::File::create(&temp_file_path).unwrap();
        temp_file.write_all(SCRIPT).unwrap();
    }

    let mut process = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(temp_file_path.clone())  // Name of the screen recording script
        .arg("-filename")
        .arg(filename)
        .arg("-timer")
        .arg(timer.to_string())  // Convert u64 to String
        .arg("-audio")
        .arg(if audio { "1" } else { "0" })  // Convert to "1" or "0"
        .arg("-openfile")
        .arg(if open_file { "1" } else { "0" })  // Convert to "1" or "0"
        .spawn()
        .unwrap();

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let pid = process.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            Command::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().await;
        });
    });

    let window_ = window.clone();
    let pid2 = process.id().unwrap();

    window.listen_global("stop", move |_event| {
        tokio::task::spawn(async move {
            Command::new("taskkill").args(&["/PID", &pid2.to_string()]).output().await;
        });
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let output = process.wait().await.unwrap();
    fs::remove_file(&temp_file_path).unwrap();
    if output.success() {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None );
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}


pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    const SCRIPT: &[u8] = include_bytes!("record_custom_script.ps1");
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("record_custom_script.ps1");

    {
        let mut temp_file = fs::File::create(&temp_file_path).unwrap();
        temp_file.write_all(SCRIPT).unwrap();
    }

    let mut process = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(temp_file_path.clone())  // Name of the screen recording script
        .arg("-filename")
        .arg(filename)
        .arg("-timer")
        .arg(timer.to_string())  // Convert u64 to String
        .arg("-audio")
        .arg(if audio { "1" } else { "0" })  // Convert to "1" or "0"
        .arg("-openfile")
        .arg(if open_file { "1" } else { "0" })  // Convert to "1" or "0"
        .spawn()
        .unwrap();

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let pid = process.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            Command::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().await;
        });
    });

    let window_ = window.clone();
    let pid2 = process.id().unwrap();

    window.listen_global("stop", move |_event| {
        tokio::task::spawn(async move {
            Command::new("taskkill").args(&["/PID", &pid2.to_string()]).output().await;
        });
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let output = process.wait().await.unwrap();
    fs::remove_file(&temp_file_path).unwrap();

    if output.success() {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}
