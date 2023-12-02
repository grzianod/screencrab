use tokio::task;
use tauri::api::process::Command;
use tauri::api::process::CommandEvent;
use tokio::process::Command as tokioCommand;
use std::os::windows::io::AsHandle;
use std::os::windows::process::CommandExt;
use std::process::Command as stdCommand;
use tauri::{Window, Manager};
use crate::utils::*;
use winapi::um::wincon::GenerateConsoleCtrlEvent;
use winapi::um::wincon::{CTRL_C_EVENT, CTRL_BREAK_EVENT, CTRL_CLOSE_EVENT};
use std::fs;
use std::thread;
use std::time::Duration;
use cpal::traits::{DeviceTrait, HostTrait};

pub async fn capture_fullscreen(window: Window, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    let index = get_current_monitor_index(&window) - 1;
    let position = get_monitor_position(&window, index);
    let monitor = window.current_monitor().unwrap().unwrap(); // Store the monitor in a variable
    let size = monitor.size();
    if timer > 0 {
        let mut sleep_command =
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

    let status = Command::new_sidecar("ffmpeg")
        .unwrap()
        .args([
            "-f", "gdigrab",
            "-offset_x", format!("{}", position.x).as_str(),
            "-offset_y", format!("{}", position.y).as_str(),
            "-video_size", format!("{}x{}", size.width, size.height).as_str(),
            "-i", "desktop",
            "-draw_mouse", if pointer { "1" } else { "0" },
            "-frames:v", "1",
            &filename.to_string()
        ])
        .status()
        .unwrap();

    let filename1 = filename.to_string();
    if status.success() {
        if !clipboard && open_file {
            window.windows().get("main_window").unwrap().minimize().unwrap();
            window.windows().get("tools").unwrap().show().unwrap();
            window.windows().get("tools").unwrap().unminimize().unwrap();
            window.emit_all("path", filename.to_string()).unwrap();
        }
        if clipboard {
            if let Err(err) = copy_to_clipboard(filename.to_string()) {
                return Response::new(Some(format!("Failed to copy on Clipboard. Screen Crab saved to {}", filename.to_string())), None);
            } else {
                fs::remove_file(filename.to_string());
                return Response::new(Some(format!("Screen Crab saved to Clipboard")), None);
            }
        } else {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
        }
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}

pub async fn capture_custom(window: Window, area: &str, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    if timer > 0 {
        let mut sleep_command =
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
    let x = parts[0].trim().parse::<i32>().unwrap();
    let y = parts[1].trim().parse::<i32>().unwrap();
    let width = parts[2].trim().parse::<i32>().unwrap();
    let height = parts[3].trim().parse::<i32>().unwrap();


    let status = Command::new_sidecar("ffmpeg")
        .unwrap()
        .args([
            "-f", "gdigrab",
            "-framerate", "30",
            "-offset_x", format!("{}", x).as_str(),
            "-offset_y", format!("{}", y).as_str(),
            "-video_size", format!("{}x{}", width, height).as_str(),
            "-i", "desktop",
            "-draw_mouse", if pointer { "1" } else { "0" },
            "-frames:v", "1",
            &filename.to_string()
        ])
        .status()
        .unwrap();


    let filename1 = filename.to_string();
    if status.success() {
        if !clipboard && open_file {
            window.windows().get("main_window").unwrap().minimize().unwrap();
            window.windows().get("tools").unwrap().show().unwrap();
            window.windows().get("tools").unwrap().unminimize().unwrap();
            window.emit_all("path", filename.to_string()).unwrap();
        }
        if clipboard {
            if let Err(err) = copy_to_clipboard(filename.to_string()) {
                return Response::new(Some(format!("Failed to copy on Clipboard. Screen Crab saved to {}", filename.to_string())), None);
            } else {
                fs::remove_file(filename.to_string());
                return Response::new(Some(format!("Screen Crab saved to Clipboard")), None);
            }
        } else {
            return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
        }
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}


pub async fn record_fullscreen(window: Window, filename: &str, timer: u64, pointer: bool, clipboard: bool, audio: bool, open_file: bool) -> Response {
    let index = get_current_monitor_index(&window) - 1;
    let position = get_monitor_position(&window, index);
    let monitor = window.current_monitor().unwrap().unwrap(); // Store the monitor in a variable
    let size = monitor.size();

    if timer > 0 {
        let mut sleep_command =
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

    let mut command = stdCommand::from(Command::new_sidecar("ffmpeg").unwrap()
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
        //command.raw_arg(["-f", "dshow", "-i", audio_string.as_str()]);
        command.raw_arg("-f");
        command.raw_arg("dshow");
        command.raw_arg("-i");
        command.raw_arg(audio_string.as_str());
    }

    command.args([&filename.to_string()]);
    
    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();
    
    let mut process = command.creation_flags(0x00000200).spawn().unwrap();
    println!("{:?}", command);
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
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}


pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    if timer > 0 {
        let mut sleep_command =
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
    let x = parts[0].trim().parse::<i32>().unwrap();
    let y = parts[1].trim().parse::<i32>().unwrap();
    let width = parts[2].trim().parse::<i32>().unwrap();
    let height = parts[3].trim().parse::<i32>().unwrap();

    let mut command = stdCommand::from(Command::new_sidecar("ffmpeg")
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

    let mut process = command.creation_flags(0x00000200).spawn().unwrap();
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
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")));
}
