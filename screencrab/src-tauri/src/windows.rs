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
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None ); }
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


pub async fn record_fullscreen(window: Window, filename: &str, timer: u64, pointer: bool, clipboard: bool, audio: bool, open_file: bool) -> Response {
    let mut process = Command::new("ffmpeg")
        .stdin(Stdio::piped())
        .arg("-y")
        .args(&["-f", "gdigrab"])
        .args(&["-i", "desktop"])
        .arg(&filename)
        .spawn()
        .unwrap();

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let stdin = process.stdin.take().unwrap();
    let stdin = Arc::new(Mutex::new(stdin));
    let pid = process.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            Command::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().await;
        });
    });

    let window_ = window.clone();
    let pid2 = process.id().unwrap();

    window.listen_global("stop", move |_event| {
            unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid as u32); }
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let output = process.wait().await.unwrap();
    let filename1 = filename.to_string();
    let output = process.wait().await.unwrap();
    if output.success() {
        if open_file {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = Command::new("Start-Process").arg(filename1.to_string()).output().await.map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e)) ));
            });
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}


pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    let parts: Vec<&str> = area.split(',').collect();
    let x = parts[0].trim().parse::<i32>().unwrap();
    let y = parts[1].trim().parse::<i32>().unwrap();
    let width = parts[2].trim().parse::<i32>().unwrap();
    let height = parts[3].trim().parse::<i32>().unwrap();

    println!("{}, {}, {}, {}", x,y,width, height);

    let mut process = Command::new("ffmpeg")
        .stdin(Stdio::piped())
        .arg("-y")
        .args(&["-f", "gdigrab"])
        .args(&["-video_size", &format!("{}x{}", width, height)])
        .args(&["-offset_x", &x.to_string()])
        .args(&["-offset_y", &y.to_string()])
        .args(&["-i", "desktop"])
        .arg(&filename)
        .spawn()
        .unwrap();

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let stdin = process.stdin.take().unwrap();
    let stdin = Arc::new(Mutex::new(stdin));
    let pid = process.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            Command::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().await;
        });
    });

    let window_ = window.clone();
    let pid2 = process.id().unwrap();

    window.listen_global("stop", move |_event| {
        unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid as u32); }
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let output = process.wait().await.unwrap();
    let filename1 = filename.to_string();
    let output = process.wait().await.unwrap();
    if output.success() {
        if open_file {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = Command::new("Start-Process").arg(filename1.to_string()).output().await.map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e)) ));
            });
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}
