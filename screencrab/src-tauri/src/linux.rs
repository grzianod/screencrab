use std::fs;
use tokio::task;
use tauri::api::process::Command;
use tauri::api::process::CommandEvent;
use tokio::process::Command as tokioCommand;
use std::process::Command as stdCommand;
use tauri::{Window, Manager};
use crate::utils::*;

pub async fn capture_fullscreen(window: Window, filename: &str, file_type: &str, timer: u64, pointer: bool, clipboard: bool, _audio: bool, open_file: bool) -> Response {
    let index = get_current_monitor_index(&window) - 1;

    if timer > 0 {
    let mut sleep_command = tokioCommand::new("sleep")
        .arg(&timer.to_string())
        .spawn()
        .unwrap();

    let pid = sleep_command.id().unwrap();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            let _output = tokioCommand::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()
                .await;
        });
    });
    let output = sleep_command.wait_with_output().await.unwrap();
    if !output.status.success() {
        return Response::new(None, Some(format!("Screen Crab cancelled")));
    }
    }

    let status = Command::new_sidecar("ffmpeg")
        .unwrap()
        .args(["-y", "-f", "x11grab", "-i", format!(":{}.0+0,0", index).as_str(), "-frames:v", "1", &filename.to_string()])
        .args(if pointer {vec!["-draw_mouse", "1"]} else {vec!["-draw_mouse", "0"]})
        .status()
        .unwrap();

    let filename1 = filename.to_string();
    if status.success() {
        if !clipboard && open_file {
            if let Err(_err) = stdCommand::new("xdg-open").arg(filename.to_string()).spawn() {
                return Response::new(Some(format!("Failed to open {}", filename.to_string())), None);
            }
        }
        if clipboard {
            if let Err(err) = copy_to_clipboard(filename.to_string()) {
                return Response::new(Some(format!("Failed to copy on Clipboard. Screen Crab saved to {}", filename.to_string())), None);
            }
            else {
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
    let index = get_current_monitor_index(&window) - 1;

    if timer > 0 {
    let sleep_command = stdCommand::new("sleep")
        .arg(&timer.to_string())
        .spawn()
        .unwrap();

    let pid = sleep_command.id();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            let _output = stdCommand::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()
                .unwrap();
        });
    });
    let output = sleep_command.wait_with_output().unwrap();
    println!("{:?}", output);
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
        .args(["-y", "-f", "x11grab", "-video_size", format!("{},{}", width, height).as_str(), "-i", format!(":{}.0+{},{}", index, x, y).as_str(), "-draw_mouse", if pointer { "true" } else { "false" }, "-frames:v", "1", &filename.to_string()])
        .args(if pointer {vec!["-draw_mouse", "1"]} else {vec!["-draw_mouse", "0"]})
        .status()
        .unwrap();

    
    let filename1 = filename.to_string();
    if status.success() {
        if !clipboard && open_file {
            if let Err(_err) = stdCommand::new("xdg-open").arg(filename.to_string()).spawn() {
                return Response::new(Some(format!("Failed to open {}", filename.to_string())), None);
            }
        }
        if clipboard {
            if let Err(err) = copy_to_clipboard(filename.to_string()) {
                return Response::new(Some(format!("Failed to copy on Clipboard. Screen Crab saved to {}", filename.to_string())), None);
            }
            else {
                fs::remove_file(filename.to_string());
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

    if timer > 0 {
    let sleep_command = stdCommand::new("sleep")
        .arg(&timer.to_string())
        .spawn()
        .unwrap();

    let pid = sleep_command.id();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            let _output = stdCommand::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()
                .unwrap();
        });
    });
    let output = sleep_command.wait_with_output().unwrap();
    if !output.status.success() {
        return Response::new(None, Some(format!("Screen Crab cancelled")));
    }
}

    let mut command = stdCommand::from(Command::new_sidecar("ffmpeg")
        .unwrap()
        .args(["-y", "-f", "x11grab", "-i", format!(":{}.0+0,0", index).as_str(), &filename.to_string()])
        .args(if audio {vec!["-f", "pulse", "-i", "default"]} else {Vec::with_capacity(0)})
    );

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let process = command.spawn().unwrap();
    let pid = process.id();
    let window_ = window.clone();
    let filename1 = filename.to_string();
    window.listen_global("stop", move |_event| {
        tokio::task::spawn(async move {
            let _output = tokioCommand::new("kill")
                .arg("-2")
                .arg(pid.to_string())
                .output()
                .await;
        });
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });
    
    let status = process.wait_with_output().unwrap().status;
    if status.code().unwrap() == 255 {
    if open_file {
        if let Err(_err) = stdCommand::new("xdg-open").arg(filename.to_string()).spawn() {
            return Response::new(Some(format!("Failed to open {}", filename.to_string())), None);
        }
    }
    return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}

pub async fn record_custom(window: Window, area: &str, filename: &str, timer: u64, _pointer: bool, _clipboard: bool, audio: bool, open_file: bool) -> Response {
    let index = get_current_monitor_index(&window) - 1;

    if timer > 0 {
    let sleep_command = stdCommand::new("sleep")
        .arg(&timer.to_string())
        .spawn()
        .unwrap();

    let pid = sleep_command.id();

    window.listen_global("kill", move |_event| {
        tokio::task::spawn(async move {
            let _output = stdCommand::new("kill")
                .arg("-9")
                .arg(pid.to_string())
                .output()
                .unwrap();
        });
    });
    let output = sleep_command.wait_with_output().unwrap();
    println!("{:?}", output);
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
        .args(["-f", "x11grab", "-video_size", format!("{},{}", width, height).as_str(), "-i", format!(":{}.0+{},{}", index, x, y).as_str(), &filename.to_string()])
        .args(if audio {vec!["-f", "pulse", "-i", "default"]} else {Vec::with_capacity(0)})
        .args(["-show_region", "1"])
    );

    window.menu_handle().get_item("stop_recording").set_enabled(true).unwrap();
    window.menu_handle().get_item("custom_record").set_enabled(false).unwrap();
    window.menu_handle().get_item("fullscreen_record").set_enabled(false).unwrap();

    let process = command.spawn().unwrap();

    let window_ = window.clone();
    let pid = process.id();
    window.listen_global("stop", move |_event| {
        tokio::task::spawn(async move {
            let _output = tokioCommand::new("kill")
                .arg("-2")  //SIGTERM
                .arg(pid.to_string())
                .output()
                .await;
        });
        window_.menu_handle().get_item("stop_recording").set_enabled(false).unwrap();
        window_.menu_handle().get_item("custom_record").set_enabled(true).unwrap();
        window_.menu_handle().get_item("fullscreen_record").set_enabled(true).unwrap();
    });

    let status = process.wait_with_output().unwrap().status;
    let filename1 = filename.to_string();
    if status.code().unwrap() == 255 {
        if open_file {
            if let Err(_err) = stdCommand::new("xdg-open").arg(filename.to_string()).spawn() {
                return Response::new(Some(format!("Failed to open {}", filename.to_string())), None);
            }
        }
        return Response::new(Some(format!("Screen Crab saved to {}", filename.to_string())), None);
    }
    return Response::new(None, Some(format!("Screen Crab cancelled")) );
}
