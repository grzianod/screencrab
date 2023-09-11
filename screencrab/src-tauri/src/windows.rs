use tauri::{Window, Manager};
use crate::utils::*;
use std::fs;
use std::io::Write;
use std::ffi::OsString;
use tokio::process::Command;
use tokio::task;
use winapi::um::wincon::{GenerateConsoleCtrlEvent};
use winapi::um::wincon::CTRL_C_EVENT;
use winapi::um::wincon::CTRL_BREAK_EVENT;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;

pub async fn capture_fullscreen(window: Window, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {

    const SCRIPT: &[u8] = include_bytes!("windows/screenshot_full_script.ps1");
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
            Command::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().await.unwrap();
        });
    });

    let mut output = process.wait().await.unwrap();
    fs::remove_file(&temp_file_path).unwrap();
    if output.success() {
        if clipboard {
            return Response::new(Some(format!("Screen Crab saved to Clipboard")), None ); }
        else {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None ); }
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}

pub async fn capture_custom(window: Window, area: &str, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {

    const SCRIPT: &[u8] = include_bytes!("windows/screenshot_custom_script.ps1");
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
            Command::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().await.unwrap();
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


pub async fn record_fullscreen(window: Window, filename: &str, timer: u64, pointer: bool, clipboard: bool, audio: bool, open_file: bool) -> Response {
    const SCRIPT: &[u8] = include_bytes!("windows/record_full_script.ps1");
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("record_full_script.ps1");

    {
        let mut temp_file = fs::File::create(&temp_file_path).unwrap();
        temp_file.write_all(SCRIPT).unwrap();
    }

    let process = Command::new("powershell")
        .creation_flags(0x00000200)
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(temp_file_path.clone())  // Name of the screen recording script
        .arg("-filename")
        .arg(filename)
        .arg("-timer")
        .arg(timer.to_string())  // Convert u64 to String
        .arg("-audio")
        .arg(if audio { "True" } else { "False" })  // Convert to "1" or "0"
        .spawn()
        .map_err(|e| Response::new( None, Some(format!("Failed to take screenshot: {}", e)) ));

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let pid = process.as_ref().unwrap().id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            Command::new("taskkill").arg("/F").arg("/PID").arg(&pid.to_string()).output().await.unwrap();
        });
    });

    let window_ = window.clone();
    window.listen_global("stop", move |_event| {
        unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid as u32); }
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let output = process.unwrap().wait().await.unwrap();
    fs::remove_file(&temp_file_path).unwrap();
    let filename1 = filename.to_string();
    if output.success() {
        if open_file {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = Command::new("cmd").arg("/C").arg(filename1.as_str()).output().await.map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e)) ));
            });
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );

}


pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    const SCRIPT: &[u8] = include_bytes!("windows/record_custom_script.ps1");
    let temp_dir = std::env::temp_dir();
    let temp_file_path = temp_dir.join("record_custom_script.ps1");

    {
        let mut temp_file = fs::File::create(&temp_file_path).unwrap();
        temp_file.write_all(SCRIPT).unwrap();
    }

    let process = Command::new("powershell")
        .creation_flags(0x00000200)
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(temp_file_path.clone())  // Name of the screen recording script
        .arg("-filename")
        .arg(filename)
        .arg("-timer")
        .arg(timer.to_string())  // Convert u64 to String
        .arg("-area")
        .arg(&area.to_string())
        .arg("-audio")
        .arg(if audio { "True" } else { "False" })  // Convert to "1" or "0"
        .spawn()
        .map_err(|e| Response::new( None, Some(format!("Failed to take screenshot: {}", e)) ));

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let pid = process.as_ref().unwrap().id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            let _ = Command::new("taskkill").arg("/F").arg("/PID").arg(&pid.to_string()).output().await.unwrap();
        });
    });

    let window_ = window.clone();
    window.listen_global("stop", move |_event| {
        unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid as u32); }
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let output = process.unwrap().wait().await.unwrap();
    fs::remove_file(&temp_file_path).unwrap();
    let filename1 = filename.to_string();
    if output.success() {
        if open_file {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = Command::new("cmd").arg("/C").arg(filename1.as_str()).output().await.map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e)) ));
            });
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}
