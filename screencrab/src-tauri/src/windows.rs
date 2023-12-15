#![windows_subsystem = "windows"]
use tokio::task;
use tauri::api::process::Command as tauriCommand;
//use tauri::api::process::CommandEvent;
use tokio::process::Command as tokioCommand;
//use std::os::windows::io::AsHandle;
use std::os::windows::process::CommandExt;
use std::process::Command as stdCommand;
use tauri::{Window, Manager};
//use std::process::{Command as StdCommand};
use crate::utils::*;
use winapi::um::wincon::GenerateConsoleCtrlEvent;
use winapi::um::wincon::CTRL_BREAK_EVENT;
//use winapi::um::wincon::{CTRL_C_EVENT, CTRL_BREAK_EVENT, CTRL_CLOSE_EVENT};
use std::fs;
use cpal::traits::{HostTrait,DeviceTrait};
use printpdf::*;

pub async fn capture_fullscreen(window: Window, filename: &str, _file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    let index = get_current_monitor_index(&window) - 1;
    let position = get_monitor_position(&window, index);
    let monitor = window.current_monitor().unwrap().unwrap(); // Store the monitor in a variable
    let size = monitor.size();
    if timer > 0 {
        let sleep_command =
            stdCommand::new("timeout")
                .arg("/t")
                .arg(&timer.to_string())
                .arg("/nobreak") // Ensures that waiting cannot be skipped with a keypress
                .spawn()
                .unwrap();

        let pid = sleep_command.id();

        window.listen_global("kill", move |_event| {
            tokio::task::spawn(async move {
                stdCommand::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().unwrap();
            });
        });

        let output = sleep_command.wait_with_output().unwrap();
        if !output.status.success() {
            return Response::new(None, Some(format!("Screen Crab cancelled")));
        }
    }

    let status = tauriCommand::new_sidecar("ffmpeg")
        .unwrap()
        .args(["-draw_mouse", if pointer {"1"} else {"0"}])
        .args([
            "-f", "gdigrab",
            "-offset_x", format!("{}", position.x).as_str(),
            "-offset_y", format!("{}", position.y).as_str(),
            "-video_size", format!("{}x{}", size.width, size.height).as_str(),
            "-i", "desktop",
            "-frames:v", "1",
            &filename.to_string()
        ])
        .status()
        .unwrap();

    if status.success() {
        if !clipboard && open_file {
            tauriCommand::new_sidecar("prtools").unwrap().args(["--path", filename]).spawn().unwrap();
            window.app_handle().windows().get("main_window").unwrap().minimize().unwrap();
        }
        if clipboard {
            if let Err(_err) = copy_to_clipboard(filename.to_string()) {
                return Response::new(Some(format!("Failed to copy on Clipboard. Screen Crab saved to {}", filename.to_string())), None);
            } else {
                let _ = fs::remove_file(filename.to_string());
                return Response::new(Some(format!("Screen Crab saved to Clipboard")), None);
            }
        } else {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
        }
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}

pub async fn capture_custom(window: Window, area: &str, filename: &str, _file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    if timer > 0 {
        let sleep_command =
            stdCommand::new("timeout")
                .arg("/t")
                .arg(&timer.to_string())
                .arg("/nobreak") // Ensures that waiting cannot be skipped with a keypress
                .spawn()
                .unwrap();

        let pid = sleep_command.id();

        window.listen_global("kill", move |_event| {
            tokio::task::spawn(async move {
                stdCommand::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().unwrap();
            });
        });

        let output = sleep_command.wait_with_output().unwrap();
        if !output.status.success() {
            return Response::new(None, Some(format!("Screen Crab cancelled")));
        }
    }

    let parts: Vec<&str> = area.split(',').collect();
    let x = (parts[0].trim().parse::<f64>().unwrap() * window.app_handle().windows().get("selector").unwrap().current_monitor().unwrap().unwrap().scale_factor()).floor() as i32;
    let y = (parts[1].trim().parse::<f64>().unwrap() * window.app_handle().windows().get("selector").unwrap().current_monitor().unwrap().unwrap().scale_factor()).floor() as i32;
    let width = (parts[2].trim().parse::<f64>().unwrap() * window.app_handle().windows().get("selector").unwrap().current_monitor().unwrap().unwrap().scale_factor()).ceil() as i32;
    let height = (parts[3].trim().parse::<f64>().unwrap()* window.app_handle().windows().get("selector").unwrap().current_monitor().unwrap().unwrap().scale_factor()).ceil() as i32;


    let status = tauriCommand::new_sidecar("ffmpeg")
        .unwrap()
        .args(["-draw_mouse", if pointer {"1"} else {"0"}])
        .args([
            "-f", "gdigrab",
            "-framerate", "30",
            "-offset_x", format!("{}", x).as_str(),
            "-offset_y", format!("{}", y).as_str(),
            "-video_size", format!("{}x{}", width, height).as_str(),
            "-i", "desktop",
            "-frames:v", "1",
            &filename.to_string()
        ])
        .status()
        .unwrap();


    if status.success() {
        if !clipboard && open_file {
            tauriCommand::new_sidecar("prtools").unwrap().args(["--path", filename]).spawn().unwrap();
            window.app_handle().windows().get("main_window").unwrap().minimize().unwrap();
        }
        if clipboard {
            if let Err(_err) = copy_to_clipboard(filename.to_string()) {
                return Response::new(Some(format!("Failed to copy on Clipboard. Screen Crab saved to {}", filename.to_string())), None);
            } else {
                let _ = fs::remove_file(filename.to_string());
                return Response::new(Some(format!("Screen Crab saved to Clipboard")), None);
            }
        } else {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
        }
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}

pub async fn record_fullscreen(window: Window, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    let index = get_current_monitor_index(&window) - 1;
    let position = get_monitor_position(&window, index);
    let monitor = window.current_monitor().unwrap().unwrap(); // Store the monitor in a variable
    let size = monitor.size();

    if timer > 0 {
        let sleep_command =
            stdCommand::new("timeout")
                .arg("/t")
                .arg(&timer.to_string())
                .arg("/nobreak") // Ensures that waiting cannot be skipped with a keypress
                .spawn()
                .unwrap();

        let pid = sleep_command.id();

        window.listen_global("kill", move |_event| {
            tokio::task::spawn(async move {
                stdCommand::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().unwrap();
            });
        });

        let output = sleep_command.wait_with_output().unwrap();
        if !output.status.success() {
            return Response::new(None, Some(format!("Screen Crab cancelled")));
        }
    }

    
    let filename_extension = std::path::Path::new(filename)
    .extension()
    .unwrap_or_default()
    .to_str()
    .unwrap_or_default()
    .to_lowercase();

    let (_video_codec, _format_option) = match filename_extension.as_str() {
        "mp4" => ("-c:v", "libx264"),
        "mov" => ("-c:v", "qtrle"),
        "avi" => ("-c:v", "mpeg4"),
        // Add more formats as needed
        _ => return Response::new(None, Some(format!("Unsupported file format: {}", filename_extension))),
    };

    let mut command = stdCommand::from(tauriCommand::new_sidecar("ffmpeg").unwrap()
        .args(["-f", "gdigrab"])
        .args(["-framerate", "30"])
        .args(["-offset_x", format!("{}", position.x).as_str()])
        .args(["-offset_y", format!("{}", position.y).as_str()])
        .args(["-video_size", format!("{}x{}", size.width, size.height).as_str()])
        .args(["-i", "desktop"])
    );
    if audio {
        let mic_name = cpal::default_host().default_input_device().unwrap().name().unwrap();
        let audio_string = format!("audio=\"{}\"", mic_name);
        command.raw_arg("-f");
        command.raw_arg("dshow");
        command.raw_arg("-i");
        command.raw_arg(audio_string.as_str());
    
    }

    command.args([&filename.to_string()]);


    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();
    
    let process = command.creation_flags(0x00000200).spawn().unwrap();
    let pid = process.id();
    let window_ = window.clone();
    let filename1 = filename.to_string();
    window.listen_global("stop", move |_event| {
        unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid as u32); }
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });


    let status = process.wait_with_output().unwrap().status;
    if status.code().unwrap() == 255 {
        if open_file {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = stdCommand::new("cmd").arg("/C").arg(filename1.as_str()).output().map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e))));
            });
            window.app_handle().windows().get("main_window").unwrap().minimize().unwrap();
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}


pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    if timer > 0 {
        let sleep_command =
            stdCommand::new("timeout")
                .arg("/t")
                .arg(&timer.to_string())
                .arg("/nobreak") // Ensures that waiting cannot be skipped with a keypress
                .spawn()
                .unwrap();

        let pid = sleep_command.id();

        window.listen_global("kill", move |_event| {
            tokio::task::spawn(async move {
                stdCommand::new("taskkill").args(&["/F", "/PID", &pid.to_string()]).output().unwrap();
            });
        });

        let output = sleep_command.wait_with_output().unwrap();
        if !output.status.success() {
            return Response::new(None, Some(format!("Screen Crab cancelled")));
        }
    }

    let parts: Vec<&str> = area.split(',').collect();
    let x = parts[0].trim().parse::<f64>().unwrap().ceil() as i32;
    let y = parts[1].trim().parse::<f64>().unwrap().ceil() as i32;
    let width = parts[2].trim().parse::<f64>().unwrap().ceil() as i32;
    let height = parts[3].trim().parse::<f64>().unwrap().ceil() as i32;

    let mut command = stdCommand::from(tauriCommand::new_sidecar("ffmpeg")
        .unwrap()
        .args(["-f", "gdigrab"])
        .args(["-framerate", "30"])
        .args(["-offset_x", format!("{}", x).as_str()])
        .args(["-offset_y", format!("{}", y).as_str()])
        .args(["-video_size", format!("{}x{}", width, height).as_str()])
        .args(["-i", "desktop"]));
        //.args(["-show_region", "1"]));

        if audio {
            let mic_name = cpal::default_host().default_input_device().unwrap().name().unwrap();
            let audio_string = format!("audio=\"{}\"", mic_name);
            //command.raw_arg(["-f", "dshow", "-i", audio_string.as_str()]);
            command.raw_arg("-f");
            command.raw_arg("dshow");
            command.raw_arg("-i");
            command.raw_arg(audio_string.as_str());
        }
    command.args(["-show_region", "1",&filename.to_string()]);

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let process = command.creation_flags(0x00000200).spawn().unwrap();
    let pid = process.id();
    let window_ = window.clone();
    let filename1 = filename.to_string();
    window.listen_global("stop", move |_event| {
        unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid as u32); }
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let status = process.wait_with_output().unwrap().status;
    if status.code().unwrap() == 255 {
        if open_file {
            // Use tokio::task::spawn to execute the opening
            let _open_task = task::spawn(async move {
                let _open = tokioCommand::new("cmd").arg("/C").arg(filename1.as_str()).output().await.map_err(|e| Response::new(None, Some(format!("Failed to open screenshot: {}", e))));
            });
            window.app_handle().windows().get("main_window").unwrap().minimize().unwrap();
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}
