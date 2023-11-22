use tauri::{Window, Manager};
use tauri::async_runtime::spawn;
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
use tauri::api::process::{CommandEvent, CommandChild};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

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

// Global or shared reference to the CommandChild
static mut FFMPEG_PROCESS: Option<CommandChild> = None;

pub fn stop_recording() {
    unsafe {
        if let Some(child) = FFMPEG_PROCESS.as_mut() {
            // Attempt to kill the process
            let _ = child.kill();
        }
    }
}

pub async fn record_fullscreen(window: Window, filename: &str, timer: u64, pointer: bool, clipboard: bool, audio: bool, open_file: bool) -> Response {
    /* 
    // first implementation with binary and cmd exec
    let ffmpeg_path = "./binaries/ffmpeg-x86_64-pc-windows-msvc.exe";
    
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
        .arg("-ffmpegPath")
        .arg(ffmpeg_path)
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
    */
    // Set up ffmpeg command arguments
    let mut ffmpeg_args = Vec::new();

    // Video capture settings
    ffmpeg_args.push("-f".to_string());
    ffmpeg_args.push("gdigrab".to_string()); // For screen capture on Windows
    ffmpeg_args.push("-i".to_string());
    ffmpeg_args.push("desktop".to_string()); // Input is the desktop

    // Audio capture settings (if audio is true)
    if audio {
        // Example: capturing default audio device
        ffmpeg_args.push("-f".to_string());
        ffmpeg_args.push("dshow".to_string()); // DirectShow for Windows
        ffmpeg_args.push("-i".to_string());
        ffmpeg_args.push("audio=\"Your Audio Device Name\"".to_string()); // Replace with actual device name
    }

    // Output file
    ffmpeg_args.push(filename.to_string()); // The output file path

    // Create sidecar command for ffmpeg
    let command = tauri::api::process::Command::new_sidecar("ffmpeg")
    .expect("Failed to create ffmpeg command");

    
    let (mut rx, mut child) = command
    .args(&ffmpeg_args)
    .spawn()
    .expect("Failed to spawn ffmpeg sidecar");

    // Shared atomic variable to control the recording
    let stop_recording = Arc::new(AtomicBool::new(false));

    // Clone the Arc to use in the async block
    let stop_clone = stop_recording.clone();

    let (_, child) = command
        .args(&ffmpeg_args)
        .spawn()
        .expect("Failed to spawn ffmpeg sidecar");

    // Store the child process in the global variable
    unsafe {
        FFMPEG_PROCESS = Some(child);
    }

    // Additional logic if necessary

    return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None );
}

async fn wait_for_stop_signal() {
    // Implement your logic to wait for a stop signal here
    // For example, wait for a user input, an event, or a timer
}

pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    let ffmpeg_path = "./binaries/ffmpeg-x86_64-pc-windows-msvc.exe";

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
        .arg("-ffmpegPath")
        .arg(ffmpeg_path)
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
